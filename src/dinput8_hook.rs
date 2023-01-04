use std::{
    ffi::c_void,
    mem::{size_of, transmute},
};

use encoding_rs::SHIFT_JIS;
use windows::{
    core::{IUnknown, GUID, HRESULT, HSTRING, PCWSTR},
    s,
    Win32::{
        Devices::HumanInterfaceDevice::{
            GUID_SysKeyboard, IDirectInput8W, IDirectInputDevice8A, DIJOYSTATE2, DISCL_FOREGROUND,
            DISCL_NONEXCLUSIVE, DISCL_NOWINKEY, DISFFC_CONTINUE,
        },
        Foundation::{FARPROC, HINSTANCE, HWND, MAX_PATH},
        System::{
            LibraryLoader::{GetProcAddress, LoadLibraryW},
            Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_READWRITE},
            SystemInformation::GetSystemDirectoryW,
        },
    },
};

use crate::modify_state::modify_state;

static mut ORIGINAL_DIRECT_INPUT8_CREATE: FARPROC = None;
static mut ORIGINAL_I_DIRECT_INPUT_8_A_CREATE_DEVICE: usize = 0;
static mut ORIGINAL_I_DIRECT_INPUT_DEVICE_8_A_GET_DEVICE_STATE: usize = 0;
static mut ORIGINAL_I_DIRECT_INPUT_DEVICE_8_A_SET_COOPERATIVE_LEVEL: usize = 0;

extern "system" fn i_direct_input_device_8_a_get_device_state_hook(
    this: *const IDirectInputDevice8A,
    cb_data: u32,
    lpv_data: *mut c_void,
) -> HRESULT {
    type Func = extern "system" fn(
        this: *const IDirectInputDevice8A,
        cb_data: u32,
        lpv_data: *mut c_void,
    ) -> HRESULT;

    let func: Func = unsafe { transmute(ORIGINAL_I_DIRECT_INPUT_DEVICE_8_A_GET_DEVICE_STATE) };
    let result = func(this, cb_data, lpv_data);
    if result.is_err() || cb_data as usize != size_of::<DIJOYSTATE2>() {
        return result;
    }
    let joy_state = lpv_data as *mut DIJOYSTATE2;
    modify_state(unsafe { joy_state.as_mut().unwrap() });
    result
}

fn write_log(msg: &str) {
    const P_LOG: *mut *mut u8 = (0x4A5940 + 0x2000) as *mut *mut u8;
    unsafe { assert!(0x4A5940 <= (*P_LOG as usize) && (*P_LOG as usize) < 0x4A5940 + 0x2000) };

    let msg = format!("{}\r\n\0", msg);
    let msg = SHIFT_JIS.encode(&msg).0;

    unsafe {
        msg.as_ptr()
            .copy_to(*P_LOG, P_LOG as usize - *P_LOG as usize);
        *(P_LOG as *mut usize) += msg.len() - 1;
        assert!(*(P_LOG as *mut usize) < P_LOG as usize);
    };
}

extern "system" fn i_direct_input_device_8_a_set_cooperative_level_hook(
    this: *const IDirectInputDevice8A,
    hwnd: HWND,
    flags: u32,
) -> HRESULT {
    type Func =
        extern "system" fn(this: *const IDirectInputDevice8A, hwnd: HWND, flags: u32) -> HRESULT;

    let func: Func = unsafe { transmute(ORIGINAL_I_DIRECT_INPUT_DEVICE_8_A_SET_COOPERATIVE_LEVEL) };
    let result = func(this, hwnd, flags);
    if result.is_err()
        && hwnd == HWND(0)
        && flags == DISCL_NOWINKEY + DISCL_FOREGROUND + DISCL_NONEXCLUSIVE
    {
        // HACK: 地霊殿のみ dinput8 の初期化に失敗する為無理矢理成功させる
        write_log("【THPovDpad】 SetCooperativeLevel() をパッチしました");

        return func(this, HWND(0), DISFFC_CONTINUE + DISCL_NONEXCLUSIVE);
    }
    result
}

fn setup_method_hook<T>(
    obj: *const T,
    method_offset: isize,
    hooked_method_addr: usize,
    original_method_addr: &mut usize,
) {
    let vtable = unsafe { *(obj as *const *mut *const c_void) };
    let mut old_protect: PAGE_PROTECTION_FLAGS = Default::default();
    let ptr = unsafe { vtable.offset(method_offset) };
    unsafe {
        VirtualProtect(
            ptr as *const c_void,
            3 * size_of::<&c_void>(),
            PAGE_READWRITE,
            &mut old_protect,
        );
        *original_method_addr = *ptr as usize;
        *ptr = hooked_method_addr as _;
        VirtualProtect(
            ptr as *const c_void,
            3 * size_of::<&c_void>(),
            old_protect,
            &mut old_protect,
        );
    }
}

extern "system" fn i_direct_input_8_a_create_device_hook(
    this: *const IDirectInput8W,
    guid: *const GUID,
    direct_input_device: *mut *const IDirectInputDevice8A,
    unk_outer: *const IUnknown,
) -> HRESULT {
    type Func = extern "system" fn(
        this: *const IDirectInput8W,
        guid: *const GUID,
        direct_input_device: *mut *const IDirectInputDevice8A,
        unk_outer: *const IUnknown,
    ) -> HRESULT;

    let func: Func = unsafe { transmute(ORIGINAL_I_DIRECT_INPUT_8_A_CREATE_DEVICE) };
    let result = func(this, guid, direct_input_device, unk_outer);
    if result.is_err() {
        return result;
    }
    if unsafe { *guid } == GUID_SysKeyboard {
        setup_method_hook(
            unsafe { *direct_input_device },
            13,
            i_direct_input_device_8_a_set_cooperative_level_hook as usize,
            unsafe { &mut ORIGINAL_I_DIRECT_INPUT_DEVICE_8_A_SET_COOPERATIVE_LEVEL },
        );
        return result;
    }
    setup_method_hook(
        unsafe { *direct_input_device },
        9,
        i_direct_input_device_8_a_get_device_state_hook as usize,
        unsafe { &mut ORIGINAL_I_DIRECT_INPUT_DEVICE_8_A_GET_DEVICE_STATE },
    );
    result
}

#[no_mangle]
pub extern "system" fn DirectInput8Create(
    inst: HINSTANCE,
    version: u32,
    riidltf: *const GUID,
    out: *mut *mut c_void,
    unkouter: IUnknown,
) -> HRESULT {
    type Func = extern "system" fn(
        inst: HINSTANCE,
        version: u32,
        riidltf: *const GUID,
        out: *mut *mut c_void,
        unkouter: IUnknown,
    ) -> HRESULT;

    let func: Func = unsafe { transmute(ORIGINAL_DIRECT_INPUT8_CREATE) };
    let result = func(inst, version, riidltf, out, unkouter);
    if result.is_err() {
        return result;
    }

    setup_method_hook(
        unsafe { *out },
        3,
        i_direct_input_8_a_create_device_hook as usize,
        unsafe { &mut ORIGINAL_I_DIRECT_INPUT_8_A_CREATE_DEVICE },
    );

    result
}

pub fn setup_dinput8_hook() {
    let system_directory = unsafe {
        let mut buf = [0u16; MAX_PATH as usize];
        GetSystemDirectoryW(Some(&mut buf));
        PCWSTR::from_raw(buf.as_mut_ptr()).to_string().unwrap()
    };
    let dll_path = format!("{}\\dinput8.dll", system_directory);
    let dll_instance = unsafe { LoadLibraryW(PCWSTR::from(&HSTRING::from(dll_path))) }.unwrap();

    if dll_instance.is_invalid() {
        panic!();
    }
    let func = unsafe { GetProcAddress(dll_instance, s!("DirectInput8Create")) };
    unsafe { ORIGINAL_DIRECT_INPUT8_CREATE = Some(func.unwrap()) };
}
