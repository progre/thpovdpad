mod custom_direct_input_8a;
mod custom_direct_input_8w;
mod custom_direct_input_device_8a;
mod custom_direct_input_device_8w;
mod dinput8_hook;
mod direct_input_device_8_delegate;
mod modify_state;

use std::ffi::c_void;

use windows::Win32::{
    Foundation::HINSTANCE,
    System::{Console::AllocConsole, SystemServices::DLL_PROCESS_ATTACH},
};

#[no_mangle]
pub extern "system" fn DllMain(
    _inst_dll: HINSTANCE,
    reason: u32,
    _reserved: *const c_void,
) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        if cfg!(debug_assertions) {
            let _ = unsafe { AllocConsole() };
        }
        dinput8_hook::setup();
    }
    true
}
