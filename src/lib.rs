use std::{ffi::c_void, iter, panic, process};

use windows::{
    core::{implement, Error, Result, PCWSTR, PWSTR},
    Win32::{
        Foundation::{BOOL, ERROR_INSUFFICIENT_BUFFER, HINSTANCE, S_FALSE, S_OK},
        System::{
            Com::{IClassFactory, IClassFactory_Impl},
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        },
        UI::Shell::{
            IShellIconOverlayIdentifier, IShellIconOverlayIdentifier_Impl, ISIOI_ICONFILE,
        },
    },
};
use windows_core::{ComObjectInner, IInspectable, IUnknown, Interface, GUID, HRESULT};

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "webm", "flv", "mov", "wmv"];

#[implement(IShellIconOverlayIdentifier)]
struct WatchedOverlay;

#[implement(IClassFactory)]
struct WatchedOverlayFactory;

impl IShellIconOverlayIdentifier_Impl for WatchedOverlay_Impl {
    fn IsMemberOf(&self, pwszpath: &PCWSTR, _dwattrib: u32) -> Result<()> {
        let path = unsafe { pwszpath.to_string()? };
        let ext = path.rsplit_once('.').unwrap_or_default().1;

        let is_video = VIDEO_EXTENSIONS.contains(&ext);
        match is_video {
            true => Ok(()),
            false => Err(Error::from_hresult(S_FALSE)),
        }
    }

    fn GetOverlayInfo(
        &self,
        pwsziconfile: PWSTR,
        cchmax: i32,
        pindex: *mut i32,
        pdwflags: *mut u32,
    ) -> Result<()> {
        let icon = "V:\\Programming\\Projects\\last-watched\\icon.ico\0";
        let icon = icon.encode_utf16().collect::<Vec<_>>();

        unsafe {
            if cchmax < icon.len() as i32 {
                return Err(Error::new(
                    ERROR_INSUFFICIENT_BUFFER.into(),
                    "Icon path too long",
                ));
            }

            *pindex = 0;
            *pdwflags = ISIOI_ICONFILE;
            pwsziconfile.as_ptr().copy_from(icon.as_ptr(), icon.len());
        }

        Ok(())
    }

    fn GetPriority(&self) -> Result<i32> {
        Ok(0)
    }
}

impl IClassFactory_Impl for WatchedOverlayFactory_Impl {
    fn CreateInstance(
        &self,
        _punkouter: Option<&IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut c_void,
    ) -> Result<()> {
        let obj: IInspectable = WatchedOverlay.into();
        unsafe { obj.query(riid, ppvobject).ok() }
    }

    fn LockServer(&self, _flock: BOOL) -> Result<()> {
        Ok(())
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let msg = $crate::to_pcstr(&format!($($arg)*));
        windows::Win32::System::Diagnostics::Debug::OutputDebugStringA(windows::core::PCSTR(msg.as_ptr()));
    }};
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
unsafe extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    reserved: *mut (),
) -> bool {
    let result = match call_reason {
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

// DllGetClassObject

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
unsafe extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let obj = WatchedOverlay.into_object();
    S_OK
}

fn to_pcstr(s: &str) -> Vec<u8> {
    s.bytes().chain(iter::once(0)).collect()
}
