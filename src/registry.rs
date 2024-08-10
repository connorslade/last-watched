use anyhow::Result;
use windows_core::GUID;
use winreg::{
    enums::{HKEY_CLASSES_ROOT, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS},
    RegKey,
};

pub fn register_clsid(module_path: &str, guid: GUID) -> Result<RegKey> {
    let guid = format!("{{{guid:?}}}");

    let clsid = RegKey::predef(HKEY_CLASSES_ROOT).open_subkey("CLSID")?;
    let (key, _) = clsid.create_subkey(&guid)?;
    key.set_value("", &"last-watched")?;

    let (inproc, _) = key.create_subkey("InProcServer32")?;
    inproc.set_value("", &module_path)?;
    inproc.set_value("ThreadingModel", &"Apartment")?;

    let (overlays, _) = RegKey::predef(HKEY_LOCAL_MACHINE).create_subkey(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers\LastWatched",
    )?;
    overlays.set_value("", &guid)?;

    Ok(key)
}

pub fn unregister_clsid(guid: GUID) -> Result<()> {
    let guid = format!("{{{guid:?}}}");

    let clsid =
        RegKey::predef(HKEY_CLASSES_ROOT).open_subkey_with_flags("CLSID", KEY_ALL_ACCESS)?;
    clsid.delete_subkey(&guid)?;

    let (overlays, _) = RegKey::predef(HKEY_LOCAL_MACHINE).create_subkey(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\ShellIconOverlayIdentifiers",
    )?;
    overlays.delete_subkey(&guid)?;

    Ok(())
}
