use std::{ffi::c_void, panic, process};

use registry::{register_clsid, unregister_clsid};
use windows::Win32::{
    Foundation::{CLASS_E_CLASSNOTAVAILABLE, HANDLE, HINSTANCE, MAX_PATH, S_OK},
    System::{
        Com::IClassFactory,
        LibraryLoader::GetModuleFileNameW,
        ProcessStatus::GetProcessImageFileNameA,
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
    UI::Shell::{SHChangeNotify, SHCNE_ASSOCCHANGED, SHCNF_IDLIST},
};
use windows_core::{Interface, GUID, HRESULT};

mod misc;
mod overlay_provider;
mod provider_factory;
mod registry;
use provider_factory::WatchedOverlayFactory;

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "webm", "flv", "mov", "wmv"];

// {172d5af2-6916-48d3-a611-368273076434}
pub const OVERLAY_CLSID: GUID = GUID::from_u128(0x172d5af2_6916_48d3_a611_368273076434);
// {88ac94e5-1a82-074b-8c44-da15204fe239}
pub const MKV_GUID: GUID = GUID::from_u128(0x88ac94e5_1a82_074b_8c44_da15204fe239);

static mut INSTANCE: HINSTANCE = HINSTANCE(0 as _);

#[no_mangle]
unsafe extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _reserved: *mut (),
) -> bool {
    let mut buf = [0u8; MAX_PATH as usize];
    let len = GetProcessImageFileNameA(HANDLE(usize::MAX as *mut c_void), &mut buf);
    let name = String::from_utf8_lossy(&buf[..len as usize]);

    match call_reason {
        DLL_PROCESS_ATTACH => {
            log!("DLL Loaded by {name}");
            INSTANCE = dll_module;
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
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    log!("DllGetClassObject {rclsid:?}");
    if *rclsid == MKV_GUID {
        let factory = IClassFactory::from(WatchedOverlayFactory);
        factory.query(riid, ppv)
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[no_mangle]
unsafe extern "system" fn DllCanUnloadNow() -> HRESULT {
    log!("DllCanUnloadNow");
    S_OK
}

#[no_mangle]
unsafe extern "system" fn DllRegisterServer() -> HRESULT {
    log!("DllRegisterServer");
    let module_path = get_module_path(INSTANCE);
    register_clsid(&module_path, OVERLAY_CLSID).unwrap();
    SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None);
    S_OK
}

#[no_mangle]
unsafe extern "system" fn DllUnregisterServer() -> HRESULT {
    log!("DllUnregisterServer");
    unregister_clsid(OVERLAY_CLSID).unwrap();
    SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, None, None);
    S_OK
}

unsafe fn get_module_path(instance: HINSTANCE) -> String {
    let mut path = [0u16; MAX_PATH as usize];
    let len = GetModuleFileNameW(instance, &mut path);
    String::from_utf16_lossy(&path[..len as usize])
}
