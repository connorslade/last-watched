use std::{fs, iter, path::Path};

use windows::{
    core::{implement, Error, Result, PCWSTR, PWSTR},
    Win32::{
        Foundation::{ERROR_INSUFFICIENT_BUFFER, S_FALSE},
        UI::Shell::{
            IShellIconOverlayIdentifier, IShellIconOverlayIdentifier_Impl, ISIOI_ICONFILE,
        },
    },
};

use crate::{log, VIDEO_EXTENSIONS};

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
            return IsMemberOfResult::Member.into();
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
        let icon = r"V:\Programming\Projects\last-watched\icon.ico";
        let icon = icon.encode_utf16().chain(iter::once(0)).collect::<Vec<_>>();

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

impl Into<Result<()>> for IsMemberOfResult {
    fn into(self) -> Result<()> {
        match self {
            IsMemberOfResult::Member => Ok(()),
            IsMemberOfResult::NotMember => Err(Error::from_hresult(S_FALSE)),
        }
    }
}
