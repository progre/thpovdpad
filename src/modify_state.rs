use windows::Win32::Devices::HumanInterfaceDevice::DIJOYSTATE;

pub fn modify_state(joy_state: &mut DIJOYSTATE) {
    let pov = joy_state.rgdwPOV[0];
    if (pov & 0x0000ffff) == 0x0000ffff {
        // neutral
        return;
    }
    if pov < 36000u32 / 16 {
        joy_state.lY = -1000; // 8
    } else if pov < 36000u32 / 16 * 3 {
        joy_state.lX = 1000; // 9
        joy_state.lY = -1000;
    } else if pov < 36000u32 / 16 * 5 {
        joy_state.lX = 1000; // 6
    } else if pov < 36000u32 / 16 * 7 {
        joy_state.lX = 1000; // 3
        joy_state.lY = 1000;
    } else if pov < 36000u32 / 16 * 9 {
        joy_state.lY = 1000; // 2
    } else if pov < 36000u32 / 16 * 11 {
        joy_state.lX = -1000; // 1
        joy_state.lY = 1000;
    } else if pov < 36000u32 / 16 * 13 {
        joy_state.lX = -1000; // 4
    } else if pov < 36000u32 / 16 * 15 {
        joy_state.lX = -1000; // 7
        joy_state.lY = -1000;
    } else {
        joy_state.lY = -1000;
    }
}
