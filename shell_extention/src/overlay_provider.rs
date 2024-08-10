use std::{fs, iter, os::windows::ffi::OsStrExt, path::Path};

use windows::{
    core::{implement, Error, Result, PCWSTR, PWSTR},
    Win32::{
        Foundation::{ERROR_INSUFFICIENT_BUFFER, S_FALSE},
        Storage::FileSystem::{
            GetFileAttributesW, SetFileAttributesW, FILE_ATTRIBUTE_HIDDEN,
            FILE_FLAGS_AND_ATTRIBUTES,
        },
        UI::Shell::{
            IShellIconOverlayIdentifier, IShellIconOverlayIdentifier_Impl, ISIOI_ICONFILE,
        },
    },
};

use crate::{
    log,
    misc::{get_module_path, to_pcwstr},
    INSTANCE, VIDEO_EXTENSIONS,
};

#[implement(IShellIconOverlayIdentifier)]
pub struct WatchedOverlay;

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
        unsafe {
            let string = to_pcwstr(&sidecar.to_string_lossy());
            let pcwstr = PCWSTR(string.as_ptr());

            let attributes = GetFileAttributesW(pcwstr);
            let is_hidden = attributes & FILE_ATTRIBUTE_HIDDEN.0 != 0;

            if !is_hidden {
                SetFileAttributesW(
                    pcwstr,
                    FILE_FLAGS_AND_ATTRIBUTES(attributes) | FILE_ATTRIBUTE_HIDDEN,
                )?;
            }
        }

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

impl From<IsMemberOfResult> for Result<()> {
    fn from(result: IsMemberOfResult) -> Result<()> {
        match result {
            IsMemberOfResult::Member => Ok(()),
            IsMemberOfResult::NotMember => Err(Error::from_hresult(S_FALSE)),
        }
    }
}
