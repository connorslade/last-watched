use anyhow::Result;
use windows_core::GUID;
use winreg::{enums::HKEY_CLASSES_ROOT, RegKey};

pub fn register_clsid(module_path: &str, guid: GUID) -> Result<RegKey> {
    let clsid = RegKey::predef(HKEY_CLASSES_ROOT).open_subkey("CLSID")?;
    let (key, _) = clsid.create_subkey(format!("{{{guid:?}}}"))?;
    key.set_value("", &"last-watched")?;

    let (inproc, _) = key.create_subkey("InprocServer32")?;
    inproc.set_value("", &module_path)?;

    Ok(key)
}

pub fn unregister_clsid(guid: GUID) -> Result<()> {
    let clsid = RegKey::predef(HKEY_CLASSES_ROOT).open_subkey("CLSID")?;
    clsid.delete_subkey(format!("{{{guid:?}}}"))?;
    Ok(())
}
