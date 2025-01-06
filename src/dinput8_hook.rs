use std::{ffi::c_void, mem::transmute, ptr::null_mut};

use windows::{
    core::{s, IUnknown, Interface, GUID, HRESULT, HSTRING, PCWSTR},
    Win32::{
        Devices::HumanInterfaceDevice::{IDirectInput8A, IDirectInput8W},
        Foundation::{FARPROC, HINSTANCE, MAX_PATH},
        System::{
            LibraryLoader::{GetProcAddress, LoadLibraryW},
            SystemInformation::GetSystemDirectoryW,
        },
    },
};

use crate::{
    custom_direct_input_8a::CustomDirectInput8A, custom_direct_input_8w::CustomDirectInput8W,
};

static mut ORIGINAL_DIRECT_INPUT8_CREATE: FARPROC = None;

#[no_mangle]
pub unsafe extern "system" fn DirectInput8Create(
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

    let mut di = null_mut();
    let result = func(inst, version, riidltf, &mut di, unkouter);
    if result.is_err() {
        return result;
    }

    *out = match unsafe { *riidltf } {
        IDirectInput8A::IID => {
            let di: Option<IDirectInput8A> = unsafe { transmute(di) };
            let custom_di = CustomDirectInput8A::new(di.unwrap());
            let di: IDirectInput8A = custom_di.into();
            di.into_raw()
        }
        IDirectInput8W::IID => {
            let di: Option<IDirectInput8W> = unsafe { transmute(di) };
            let custom_di = CustomDirectInput8W::new(di.unwrap());
            let di: IDirectInput8W = custom_di.into();
            di.into_raw()
        }
        _ => panic!(),
    };
    result
}

pub fn setup() {
    let system_directory = unsafe {
        let mut buf = [0u16; MAX_PATH as usize];
        GetSystemDirectoryW(Some(&mut buf));
        PCWSTR::from_raw(buf.as_ptr()).to_string().unwrap()
    };
    let dll_path = format!("{}\\dinput8.dll", system_directory);
    let dll_instance =
        unsafe { LoadLibraryW(PCWSTR::from_raw(HSTRING::from(dll_path).as_wide().as_ptr())) }
            .unwrap();

    if dll_instance.is_invalid() {
        panic!();
    }
    let func = unsafe { GetProcAddress(dll_instance, s!("DirectInput8Create")) };
    unsafe { ORIGINAL_DIRECT_INPUT8_CREATE = Some(func.unwrap()) };
}
