use std::{ffi::c_void, panic, process};

use windows::Win32::{
    Foundation::{HINSTANCE, S_OK},
    System::{
        Com::IClassFactory,
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};
use windows_core::{Interface, GUID, HRESULT};

mod misc;
mod overlay_provider;
mod provider_factory;
use provider_factory::WatchedOverlayFactory;

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "webm", "flv", "mov", "wmv"];

#[no_mangle]
unsafe extern "system" fn DllMain(
    _dll_module: HINSTANCE,
    call_reason: u32,
    _reserved: *mut (),
) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            log!("DLL Injected");
            panic::set_hook(Box::new(|info| {
                log!("== Panic ==");
                log!("{info}");
                process::abort();
            }));
        }
        DLL_PROCESS_DETACH => {
            log!("DLL Unloaded");
        }
        _ => return true,
    };

    true
}

#[no_mangle]
unsafe extern "system" fn DllGetClassObject(
    _rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let factory = IClassFactory::from(WatchedOverlayFactory);
    factory.query(riid, ppv)
}

#[no_mangle]
unsafe extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_OK
}
