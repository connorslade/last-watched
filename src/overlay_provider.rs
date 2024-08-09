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

impl IShellIconOverlayIdentifier_Impl for WatchedOverlay_Impl {
    fn IsMemberOf(&self, pwszpath: &PCWSTR, _dwattrib: u32) -> Result<()> {
        let path = unsafe { pwszpath.to_string()? };
        let ext = path.rsplit_once('.').unwrap_or_default().1;
        log!("IsMemberOf {path} {ext}");

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
        log!("GetOverlayInfo");
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
