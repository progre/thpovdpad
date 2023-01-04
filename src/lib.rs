mod dinput8_hook;
mod modify_state;

use dinput8_hook::setup_dinput8_hook;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

#[no_mangle]
pub extern "system" fn DllMain(_module: u32, reason: u32, _: u32) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        setup_dinput8_hook();
    }
    true
}
