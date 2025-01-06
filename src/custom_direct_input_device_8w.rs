use std::ffi::c_void;

use windows::Win32::{
    Devices::HumanInterfaceDevice::{
        IDirectInputDevice8W, IDirectInputDevice8W_Impl, IDirectInputEffect, DIACTIONFORMATW,
        DIDATAFORMAT, DIDEVCAPS, DIDEVICEIMAGEINFOHEADERW, DIDEVICEINSTANCEW, DIDEVICEOBJECTDATA,
        DIDEVICEOBJECTINSTANCEW, DIEFFECT, DIEFFECTINFOW, DIEFFESCAPE, DIFILEEFFECT, DIPROPHEADER,
        LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, LPDIENUMDEVICEOBJECTSCALLBACKW,
        LPDIENUMEFFECTSCALLBACKW, LPDIENUMEFFECTSINFILECALLBACK,
    },
    Foundation::{HANDLE, HINSTANCE, HWND},
};
use windows_core::{implement, GUID, PCWSTR};

use crate::direct_input_device_8_delegate::DirectInputDevice8Delegate;

#[implement(IDirectInputDevice8W)]
pub struct CustomDirectInputDevice8W {
    instance: IDirectInputDevice8W,
    delegate: DirectInputDevice8Delegate,
}

impl CustomDirectInputDevice8W {
    pub fn new(instance: IDirectInputDevice8W) -> Self {
        Self {
            instance,
            delegate: DirectInputDevice8Delegate,
        }
    }
}

impl IDirectInputDevice8W_Impl for CustomDirectInputDevice8W_Impl {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()> {
        unsafe { self.instance.GetCapabilities(param0) }
    }

    fn EnumObjects(
        &self,
        param0: LPDIENUMDEVICEOBJECTSCALLBACKW,
        param1: *mut c_void,
        param2: u32,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.EnumObjects(param0, param1, param2) }
    }

    fn GetProperty(
        &self,
        param0: *const GUID,
        param1: *mut DIPROPHEADER,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.GetProperty(param0, param1) }
    }

    fn SetProperty(
        &self,
        param0: *const GUID,
        param1: *mut DIPROPHEADER,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.SetProperty(param0, param1) }
    }

    fn Acquire(&self) -> windows_core::Result<()> {
        unsafe { self.instance.Acquire() }
    }

    fn Unacquire(&self) -> windows_core::Result<()> {
        unsafe { self.instance.Unacquire() }
    }

    fn GetDeviceState(
        &self,
        param0: u32,
        param1: *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        let result = unsafe { self.instance.GetDeviceState(param0, param1) };
        self.delegate.on_get_device_state(param0, param1, result)
    }

    fn GetDeviceData(
        &self,
        param0: u32,
        param1: *mut DIDEVICEOBJECTDATA,
        param2: *mut u32,
        param3: u32,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.GetDeviceData(param0, param1, param2, param3) }
    }

    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()> {
        unsafe { self.instance.SetDataFormat(param0) }
    }

    fn SetEventNotification(&self, param0: HANDLE) -> windows_core::Result<()> {
        unsafe { self.instance.SetEventNotification(param0) }
    }

    fn SetCooperativeLevel(&self, param0: HWND, param1: u32) -> windows_core::Result<()> {
        let result = unsafe { self.instance.SetCooperativeLevel(param0, param1) };
        self.delegate.on_set_cooperative_level_hook(
            |param0: HWND, param1: u32| unsafe {
                self.instance.SetCooperativeLevel(param0, param1)
            },
            param0,
            param1,
            result,
        )
    }

    fn GetObjectInfo(
        &self,
        param0: *mut DIDEVICEOBJECTINSTANCEW,
        param1: u32,
        param2: u32,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.GetObjectInfo(param0, param1, param2) }
    }

    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> windows_core::Result<()> {
        unsafe { self.instance.GetDeviceInfo(param0) }
    }

    fn RunControlPanel(&self, param0: HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { self.instance.RunControlPanel(param0, param1) }
    }

    fn Initialize(
        &self,
        param0: HINSTANCE,
        param1: u32,
        param2: *const GUID,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.Initialize(param0, param1, param2) }
    }

    fn CreateEffect(
        &self,
        param0: *const GUID,
        param1: *mut DIEFFECT,
        param2: *mut Option<IDirectInputEffect>,
        param3: Option<&windows_core::IUnknown>,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.CreateEffect(param0, param1, param2, param3) }
    }

    fn EnumEffects(
        &self,
        param0: LPDIENUMEFFECTSCALLBACKW,
        param1: *mut core::ffi::c_void,
        param2: u32,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.EnumEffects(param0, param1, param2) }
    }

    fn GetEffectInfo(
        &self,
        param0: *mut DIEFFECTINFOW,
        param1: *const GUID,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.GetEffectInfo(param0, param1) }
    }

    fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()> {
        unsafe { self.instance.GetForceFeedbackState(param0) }
    }

    fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()> {
        unsafe { self.instance.SendForceFeedbackCommand(param0) }
    }

    fn EnumCreatedEffectObjects(
        &self,
        param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK,
        param1: *mut core::ffi::c_void,
        param2: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            self.instance
                .EnumCreatedEffectObjects(param0, param1, param2)
        }
    }

    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()> {
        unsafe { self.instance.Escape(param0) }
    }

    fn Poll(&self) -> windows_core::Result<()> {
        unsafe { self.instance.Poll() }
    }

    fn SendDeviceData(
        &self,
        param0: u32,
        param1: *mut DIDEVICEOBJECTDATA,
        param2: *mut u32,
        param3: u32,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.SendDeviceData(param0, param1, param2, param3) }
    }

    fn EnumEffectsInFile(
        &self,
        param0: &PCWSTR,
        param1: LPDIENUMEFFECTSINFILECALLBACK,
        param2: *mut core::ffi::c_void,
        param3: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            self.instance
                .EnumEffectsInFile(*param0, param1, param2, param3)
        }
    }

    fn WriteEffectToFile(
        &self,
        param0: &PCWSTR,
        param1: u32,
        param2: *mut DIFILEEFFECT,
        param3: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            self.instance
                .WriteEffectToFile(*param0, param1, param2, param3)
        }
    }

    fn BuildActionMap(
        &self,
        param0: *mut DIACTIONFORMATW,
        param1: &PCWSTR,
        param2: u32,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.BuildActionMap(param0, *param1, param2) }
    }

    fn SetActionMap(
        &self,
        param0: *mut DIACTIONFORMATW,
        param1: &PCWSTR,
        param2: u32,
    ) -> windows_core::Result<()> {
        unsafe { self.instance.SetActionMap(param0, *param1, param2) }
    }

    fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> windows_core::Result<()> {
        unsafe { self.instance.GetImageInfo(param0) }
    }
}
