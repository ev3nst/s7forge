use winreg::RegKey;
use winreg::enums::*;

pub fn steam_install_paths() -> Result<Vec<String>, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let mut paths = Vec::new();

    if let Ok(steam_key) = hkcu.open_subkey("SOFTWARE\\Valve\\Steam") {
        if let Ok(steam_path) = steam_key.get_value::<String, _>("SteamPath") {
            paths.push(steam_path);
        }
    }

    if let Ok(steam_key) = hkcu.open_subkey("SOFTWARE\\Wow6432Node\\Valve\\Steam") {
        if let Ok(steam_path) = steam_key.get_value::<String, _>("SteamPath") {
            paths.push(steam_path);
        }
    }

    Ok(paths)
}
