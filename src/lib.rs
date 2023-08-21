mod dinput8_hook;
mod modify_state;

use std::ffi::c_void;

use windows::Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH};

#[no_mangle]
pub extern "system" fn DllMain(
    _inst_dll: HINSTANCE,
    reason: u32,
    _reserved: *const c_void,
) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        dinput8_hook::setup();
    }
    true
}
