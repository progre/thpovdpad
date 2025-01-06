use std::{ffi::c_void, fs::File, ptr::null_mut};

use encoding_rs::SHIFT_JIS;
use windows::Win32::{
    Devices::HumanInterfaceDevice::{
        DIJOYSTATE, DIJOYSTATE2, DISCL_BACKGROUND, DISCL_FOREGROUND, DISCL_NONEXCLUSIVE,
        DISCL_NOWINKEY,
    },
    Foundation::{HINSTANCE, HWND, MAX_PATH},
    System::LibraryLoader::GetModuleFileNameW,
};
use windows_core::PCWSTR;

use crate::modify_state::modify_state;

fn write_log(msg: &str) {
    const TH11_EXE_SIZE: u64 = 829584;

    let filepath = unsafe {
        let mut buf = [0u16; MAX_PATH as usize];
        GetModuleFileNameW(HINSTANCE(null_mut()), &mut buf);
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

pub struct DirectInputDevice8Delegate;

impl DirectInputDevice8Delegate {
    pub fn on_get_device_state(
        &self,
        cb_data: u32,
        data: *mut c_void,
        result: windows_core::Result<()>,
    ) -> windows_core::Result<()> {
        if result.is_err()
            || ![size_of::<DIJOYSTATE>(), size_of::<DIJOYSTATE2>()].contains(&(cb_data as usize))
        {
            return result;
        }
        let joy_state = data as *mut DIJOYSTATE;
        modify_state(unsafe { joy_state.as_mut().unwrap() });
        result
    }

    pub fn on_set_cooperative_level_hook(
        &self,
        i_direct_input_device_8_set_cooperative_level: impl FnOnce(
            HWND,
            u32,
        ) -> windows_core::Result<()>,
        hwnd: HWND,
        flags: u32,
        result: windows_core::Result<()>,
    ) -> windows_core::Result<()> {
        if result.is_ok()
            || hwnd != HWND(null_mut())
            || flags != DISCL_NOWINKEY + DISCL_FOREGROUND + DISCL_NONEXCLUSIVE
        {
            return result;
        }

        // HACK: 地霊殿のみ dinput8 の初期化に失敗する為無理矢理成功させる
        write_log("【THPovDpad】 SetCooperativeLevel() をパッチします");
        i_direct_input_device_8_set_cooperative_level(hwnd, DISCL_BACKGROUND + DISCL_NONEXCLUSIVE)
    }
}
