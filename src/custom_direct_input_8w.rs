use windows::Win32::{
    Devices::HumanInterfaceDevice::{
        IDirectInput8W, IDirectInput8W_Impl, IDirectInputDevice8W, DIACTIONFORMATW,
        DICONFIGUREDEVICESPARAMSW, LPDICONFIGUREDEVICESCALLBACK, LPDIENUMDEVICESBYSEMANTICSCBW,
        LPDIENUMDEVICESCALLBACKW,
    },
    Foundation::{HINSTANCE, HWND},
};
use windows_core::implement;

use crate::custom_direct_input_device_8w::CustomDirectInputDevice8W;

#[implement(IDirectInput8W)]
pub struct CustomDirectInput8W {
    instance: IDirectInput8W,
}

impl CustomDirectInput8W {
    pub fn new(instance: IDirectInput8W) -> Self {
        Self { instance }
    }
}

impl IDirectInput8W_Impl for CustomDirectInput8W_Impl {
    fn CreateDevice(
        &self,
        param0: *const windows_core::GUID,
        param1: *mut Option<IDirectInputDevice8W>,
        param2: Option<&windows_core::IUnknown>,
    ) -> windows_core::Result<()> {
        let mut did: Option<IDirectInputDevice8W> = None;
        unsafe { self.instance.CreateDevice(param0, &mut did, param2) }?;
        let did = CustomDirectInputDevice8W::new(did.unwrap());
        let did: IDirectInputDevice8W = did.into();
        unsafe { *param1 = Some(did) };
        Ok(())
    }

    fn EnumDevices(
        &self,
        param0: u32,
        param1: LPDIENUMDEVICESCALLBACKW,
        param2: *mut core::ffi::c_void,
        param3: u32,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.EnumDevices(param0, param1, param2, param3) }
    }

    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { self.instance.GetDeviceStatus(param0) }
    }

    fn RunControlPanel(&self, param0: HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { self.instance.RunControlPanel(param0, param1) }
    }

    fn Initialize(&self, param0: HINSTANCE, param1: u32) -> windows_core::Result<()> {
        unsafe { self.instance.Initialize(param0, param1) }
    }

    fn FindDevice(
        &self,
        param0: *const windows_core::GUID,
        param1: &windows_core::PCWSTR,
        param2: *mut windows_core::GUID,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.FindDevice(param0, *param1, param2) }
    }

    fn EnumDevicesBySemantics(
        &self,
        param0: &windows_core::PCWSTR,
        param1: *mut DIACTIONFORMATW,
        param2: LPDIENUMDEVICESBYSEMANTICSCBW,
        param3: *mut core::ffi::c_void,
        param4: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            self.instance
                .EnumDevicesBySemantics(*param0, param1, param2, param3, param4)
        }
    }

    fn ConfigureDevices(
        &self,
        param0: LPDICONFIGUREDEVICESCALLBACK,
        param1: *mut DICONFIGUREDEVICESPARAMSW,
        param2: u32,
        param3: *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            self.instance
                .ConfigureDevices(param0, param1, param2, param3)
        }
    }
}
