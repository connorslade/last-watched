use std::ffi::c_void;

use windows::{
    core::{implement, Result},
    Win32::{
        Foundation::BOOL,
        System::Com::{IClassFactory, IClassFactory_Impl},
    },
};
use windows_core::{IInspectable, IUnknown, Interface, GUID};

use crate::overlay_provider::WatchedOverlay;

#[implement(IClassFactory)]
pub struct WatchedOverlayFactory;

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
