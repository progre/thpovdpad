use std::{
    ffi::c_void,
    fs::File,
    mem::{size_of, transmute},
};

use encoding_rs::SHIFT_JIS;
use windows::{
    core::{IUnknown, Interface, GUID, HRESULT, HSTRING, PCWSTR},
    s,
    Win32::{
        Devices::HumanInterfaceDevice::{
            IDirectInput8A, IDirectInputDevice8A, DIJOYSTATE, DIJOYSTATE2, DISCL_BACKGROUND,
            DISCL_FOREGROUND, DISCL_NONEXCLUSIVE, DISCL_NOWINKEY,
        },
        Foundation::{FARPROC, HINSTANCE, HWND, MAX_PATH},
        System::{
            LibraryLoader::{GetModuleFileNameW, GetProcAddress, LoadLibraryW},
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
    data: *mut c_void,
) -> HRESULT {
    type Func = extern "system" fn(
        this: *const IDirectInputDevice8A,
        cb_data: u32,
        lpv_data: *mut c_void,
    ) -> HRESULT;
    let func: Func = unsafe { transmute(ORIGINAL_I_DIRECT_INPUT_DEVICE_8_A_GET_DEVICE_STATE) };
    let result = func(this, cb_data, data);

    if result.is_err()
        || ![size_of::<DIJOYSTATE>(), size_of::<DIJOYSTATE2>()].contains(&(cb_data as usize))
    {
        return result;
    }
    let joy_state = data as *mut DIJOYSTATE;
    modify_state(unsafe { joy_state.as_mut().unwrap() });
    result
}

fn write_log(msg: &str) {
    const TH11_EXE_SIZE: u64 = 829584;

    let filepath = unsafe {
        let mut buf = [0u16; MAX_PATH as usize];
        GetModuleFileNameW(HINSTANCE(0), &mut buf);
        PCWSTR::from_raw(buf.as_ptr()).to_string().unwrap()
    };

    if File::open(filepath).unwrap().metadata().unwrap().len() == TH11_EXE_SIZE {
        const LOG_HEAD_ADDR: usize = 0x004A5940;
        const LOG_TAIL: *mut *mut u8 = (LOG_HEAD_ADDR + 0x2000) as _;
        let valid_addr_range = LOG_HEAD_ADDR..(LOG_TAIL as usize);

        let current_tail = unsafe { *LOG_TAIL };
        assert!(valid_addr_range.contains(&(current_tail as usize)));
        let capacity = LOG_TAIL as usize - current_tail as usize;

        let msg = format!("{}\r\n\0", msg);
        let (msg, _, _) = SHIFT_JIS.encode(&msg);
        let new_tail = unsafe { current_tail.add(msg.len() - 1) };
        assert!(valid_addr_range.contains(&(new_tail as usize)));

        unsafe { msg.as_ptr().copy_to(current_tail, capacity) };
        unsafe { *LOG_TAIL = new_tail };
    } else {
        println!("{}", msg);
    }
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

    if result.is_ok()
        || hwnd != HWND(0)
        || flags != DISCL_NOWINKEY + DISCL_FOREGROUND + DISCL_NONEXCLUSIVE
    {
        return result;
    }
    // HACK: 地霊殿のみ dinput8 の初期化に失敗する為無理矢理成功させる
    write_log("【THPovDpad】 SetCooperativeLevel() をパッチします");

    func(this, HWND(0), DISCL_BACKGROUND + DISCL_NONEXCLUSIVE)
}

unsafe fn setup_method_hook(
    vtable: *mut usize,
    method_offset: isize,
    hooked_method_addr: usize,
    original_method_addr: &mut usize,
) {
    let method_addr = vtable.offset(method_offset);
    *original_method_addr = *method_addr;
    *method_addr = hooked_method_addr;
}

extern "system" fn i_direct_input_8_a_create_device_hook(
    this: *const IDirectInput8A,
    guid: *const GUID,
    out_direct_input_device: *mut *mut IDirectInputDevice8A,
    unk_outer: *const IUnknown,
) -> HRESULT {
    type Func = extern "system" fn(
        this: *const IDirectInput8A,
        guid: *const GUID,
        out_direct_input_device: *mut *mut IDirectInputDevice8A,
        unk_outer: *const IUnknown,
    ) -> HRESULT;
    let func: Func = unsafe { transmute(ORIGINAL_I_DIRECT_INPUT_8_A_CREATE_DEVICE) };
    let result = func(this, guid, out_direct_input_device, unk_outer);

    if result.is_err() {
        return result;
    }
    if unsafe { ORIGINAL_I_DIRECT_INPUT_DEVICE_8_A_GET_DEVICE_STATE } == 0 {
        unsafe {
            let direct_input_device = *out_direct_input_device;
            let vtable = *(direct_input_device as *const _);
            setup_method_hook(
                vtable,
                9,
                i_direct_input_device_8_a_get_device_state_hook as _,
                &mut ORIGINAL_I_DIRECT_INPUT_DEVICE_8_A_GET_DEVICE_STATE,
            );
            setup_method_hook(
                vtable,
                13,
                i_direct_input_device_8_a_set_cooperative_level_hook as _,
                &mut ORIGINAL_I_DIRECT_INPUT_DEVICE_8_A_SET_COOPERATIVE_LEVEL,
            );
        }
    }

    result
}

#[no_mangle]
pub extern "system" fn DirectInput8Create(
    inst: HINSTANCE,
    version: u32,
    riidltf: *const GUID,
    out: *mut *mut c_void,
    unkouter: *const IUnknown,
) -> HRESULT {
    type Func = extern "system" fn(
        inst: HINSTANCE,
        version: u32,
        riidltf: *const GUID,
        out: *mut *mut c_void,
        unkouter: *const IUnknown,
    ) -> HRESULT;
    let func: Func = unsafe { transmute(ORIGINAL_DIRECT_INPUT8_CREATE) };
    let result = func(inst, version, riidltf, out, unkouter);

    if result.is_err() || unsafe { *riidltf } != IDirectInput8A::IID {
        return result;
    }
    // NOTE: vtable はコンストラクタ―から生成される全てのインスタンスで共通 (1敗)
    if unsafe { ORIGINAL_I_DIRECT_INPUT_8_A_CREATE_DEVICE } == 0 {
        unsafe {
            let direct_input = *out;
            let vtable = *(direct_input as *const _);
            setup_method_hook(
                vtable,
                3,
                i_direct_input_8_a_create_device_hook as _,
                &mut ORIGINAL_I_DIRECT_INPUT_8_A_CREATE_DEVICE,
            );
        }
    }

    result
}

pub fn setup_dinput8_hook() {
    let system_directory = unsafe {
        let mut buf = [0u16; MAX_PATH as usize];
        GetSystemDirectoryW(Some(&mut buf));
        PCWSTR::from_raw(buf.as_ptr()).to_string().unwrap()
    };
    let dll_path = format!("{}\\dinput8.dll", system_directory);
    let dll_instance = unsafe { LoadLibraryW(PCWSTR::from(&HSTRING::from(dll_path))) }.unwrap();

    if dll_instance.is_invalid() {
        panic!();
    }
    let func = unsafe { GetProcAddress(dll_instance, s!("DirectInput8Create")) };
    unsafe { ORIGINAL_DIRECT_INPUT8_CREATE = Some(func.unwrap()) };
}
