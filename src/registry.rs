use anyhow::Result;
use windows_core::GUID;
use winreg::{
    enums::{HKEY_CLASSES_ROOT, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS, KEY_WRITE},
    RegKey,
};

pub fn register_clsid(module_path: &str, guid: GUID) -> Result<RegKey> {
    let guid = format!("{{{guid:?}}}");

    // HKEY_CLASSES_ROOT\CLSID\{guid}
    let clsid = RegKey::predef(HKEY_CLASSES_ROOT).open_subkey("CLSID")?;
    let (key, _) = clsid.create_subkey(&guid)?;
    key.set_value("", &"last-watched")?;

    let (inproc, _) = key.create_subkey("InProcServer32")?;
    inproc.set_value("", &module_path)?;
    inproc.set_value("ThreadingModel", &"Apartment")?;

    // HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Approved
    let approved = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(
        r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Approved",
        KEY_WRITE,
    )?;
    approved.set_value(&guid, &"last-watched")?;

    Ok(key)
}

pub fn unregister_clsid(guid: GUID) -> Result<()> {
    let clsid =
        RegKey::predef(HKEY_CLASSES_ROOT).open_subkey_with_flags("CLSID", KEY_ALL_ACCESS)?;
    clsid.delete_subkey(format!("{{{guid:?}}}"))?;
    Ok(())
}
