use std::{ffi::c_void, fs, iter, os::windows::ffi::OsStrExt, path::Path};

use windows::{
    core::{implement, Error, Result, PCWSTR, PWSTR},
    Win32::{
        Foundation::{BOOL, ERROR_INSUFFICIENT_BUFFER, S_FALSE},
        System::Com::{IClassFactory, IClassFactory_Impl},
        UI::Shell::{
            IShellIconOverlayIdentifier, IShellIconOverlayIdentifier_Impl, ISIOI_ICONFILE,
        },
    },
};
use windows_core::{IInspectable, IUnknown, Interface, GUID};
use winreg::{
    enums::{HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS},
    RegKey,
};

use crate::{
    log,
    misc::get_module_path,
    registry::{format_guid, register_clsid, unregister_clsid},
    INSTANCE,
};
use common::{winapi::ensure_hidden, VIDEO_EXTENSIONS};

// {172d5af2-6916-48d3-a611-368273076434}
pub const OVERLAY_CLSID: GUID = GUID::from_u128(0x172d5af2_6916_48d3_a611_368273076434);

#[implement(IShellIconOverlayIdentifier)]
pub struct WatchedOverlay;

#[implement(IClassFactory)]
pub struct WatchedOverlayFactory;

enum IsMemberOfResult {
    Member,
    NotMember,
}

impl IShellIconOverlayIdentifier_Impl for WatchedOverlay_Impl {
    fn IsMemberOf(&self, pwszpath: &PCWSTR, _dwattrib: u32) -> Result<()> {
        let path = unsafe { pwszpath.to_string()? };
        let ext = path.rsplit_once('.').unwrap_or_default().1;

        if !VIDEO_EXTENSIONS.contains(&ext) {
            return IsMemberOfResult::NotMember.into();
        }

        let path = Path::new(&path);
        let Some(parent) = path.parent() else {
            return IsMemberOfResult::NotMember.into();
        };

        let sidecar = parent.join(".watched");
        if !sidecar.exists() {
            return IsMemberOfResult::NotMember.into();
        }

        // Make sure the sidecar is hidden
        let _ = ensure_hidden(&sidecar);

        // TODO: cache the content
        let file = fs::read_to_string(sidecar)?;
        let filename = path.file_name().unwrap();
        if file.lines().any(|line| line == filename) {
            return IsMemberOfResult::Member.into();
        }

        IsMemberOfResult::NotMember.into()
    }

    fn GetOverlayInfo(
        &self,
        pwsziconfile: PWSTR,
        cchmax: i32,
        pindex: *mut i32,
        pdwflags: *mut u32,
    ) -> Result<()> {
        log!("GetOverlayInfo");
        let icon = Path::new(&unsafe { get_module_path(INSTANCE) })
            .parent()
            .unwrap()
            .join("icon.ico");
        let icon = icon
            .as_os_str()
            .encode_wide()
            .chain(iter::once(0))
            .collect::<Vec<_>>();

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
        let obj = IInspectable::from(WatchedOverlay);
        unsafe { obj.query(riid, ppvobject).ok() }
    }

    fn LockServer(&self, _flock: BOOL) -> Result<()> {
        Ok(())
    }
}

impl From<IsMemberOfResult> for Result<()> {
    fn from(result: IsMemberOfResult) -> Result<()> {
        match result {
            IsMemberOfResult::Member => Ok(()),
            IsMemberOfResult::NotMember => Err(Error::from_hresult(S_FALSE)),
        }
    }
}

pub fn register(module_path: &str) -> anyhow::Result<()> {
    register_clsid(module_path, OVERLAY_CLSID)?;

    let (overlays, _) = RegKey::predef(HKEY_LOCAL_MACHINE).create_subkey(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\LastWatched",
    )?;
    overlays.set_value("", &format_guid(&OVERLAY_CLSID))?;

    Ok(())
}

pub fn unregister() -> anyhow::Result<()> {
    unregister_clsid(OVERLAY_CLSID)?;

    let overlays = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers",
        KEY_ALL_ACCESS,
    )?;
    overlays.delete_subkey_all("LastWatched")?;

    Ok(())
}
