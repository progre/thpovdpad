use windows::Win32::{
    Devices::HumanInterfaceDevice::{
        IDirectInput8A, IDirectInput8A_Impl, IDirectInputDevice8A, DIACTIONFORMATA,
        DICONFIGUREDEVICESPARAMSA, LPDICONFIGUREDEVICESCALLBACK, LPDIENUMDEVICESBYSEMANTICSCBA,
        LPDIENUMDEVICESCALLBACKA,
    },
    Foundation::{HINSTANCE, HWND},
};
use windows_core::implement;

use crate::custom_direct_input_device_8a::CustomDirectInputDevice8A;

#[implement(IDirectInput8A)]
pub struct CustomDirectInput8A {
    instance: IDirectInput8A,
}

impl CustomDirectInput8A {
    pub fn new(instance: IDirectInput8A) -> Self {
        println!("new {instance:x?}");
        Self { instance }
    }
}

impl IDirectInput8A_Impl for CustomDirectInput8A_Impl {
    fn CreateDevice(
        &self,
        param0: *const windows_core::GUID,
        param1: *mut Option<IDirectInputDevice8A>,
        param2: Option<&windows_core::IUnknown>,
    ) -> windows_core::Result<()> {
        let mut did: Option<IDirectInputDevice8A> = None;
        unsafe { self.instance.CreateDevice(param0, &mut did, param2) }?;
        let did = CustomDirectInputDevice8A::new(did.unwrap());
        let did: IDirectInputDevice8A = did.into();
        unsafe { *param1 = Some(did) };
        Ok(())
    }

    fn EnumDevices(
        &self,
        param0: u32,
        param1: LPDIENUMDEVICESCALLBACKA,
        param2: *mut core::ffi::c_void,
        param3: u32,
    ) -> windows_core::Result<()> {
        println!("EnumDevices {:x?} {} {}", self.instance, param0, param3);
        unsafe { self.instance.EnumDevices(param0, param1, param2, param3) }
    }

    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()> {
        println!("GetDeviceStatus {:x?}", self.instance);
        unsafe { self.instance.GetDeviceStatus(param0) }
    }

    fn RunControlPanel(&self, param0: HWND, param1: u32) -> windows_core::Result<()> {
        println!("RunControlPanel {:x?}", self.instance);
        unsafe { self.instance.RunControlPanel(param0, param1) }
    }

    fn Initialize(&self, param0: HINSTANCE, param1: u32) -> windows_core::Result<()> {
        println!("Initialize {:x?}", self.instance);
        unsafe { self.instance.Initialize(param0, param1) }
    }

    fn FindDevice(
        &self,
        param0: *const windows_core::GUID,
        param1: &windows_core::PCSTR,
        param2: *mut windows_core::GUID,
    ) -> windows_core::Result<()> {
        println!("FindDevice {:x?}", self.instance);
        unsafe { self.instance.FindDevice(param0, *param1, param2) }
    }

    fn EnumDevicesBySemantics(
        &self,
        param0: &windows_core::PCSTR,
        param1: *mut DIACTIONFORMATA,
        param2: LPDIENUMDEVICESBYSEMANTICSCBA,
        param3: *mut core::ffi::c_void,
        param4: u32,
    ) -> windows_core::Result<()> {
        println!("EnumDevicesBySemantics {:x?}", self.instance);
        unsafe {
            self.instance
                .EnumDevicesBySemantics(*param0, param1, param2, param3, param4)
        }
    }

    fn ConfigureDevices(
        &self,
        param0: LPDICONFIGUREDEVICESCALLBACK,
        param1: *mut DICONFIGUREDEVICESPARAMSA,
        param2: u32,
        param3: *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        println!("ConfigureDevices {:x?}", self.instance);
        unsafe {
            self.instance
                .ConfigureDevices(param0, param1, param2, param3)
        }
    }
}
