use regex::Regex;
use std::{fs, path::Path};

use crate::core::steam_install_paths::steam_install_paths;

pub fn steam_library_paths() -> Result<Vec<String>, String> {
    let steam_install_paths = steam_install_paths()?;
    let mut library_folder_paths = Vec::new();
    let re = Regex::new(r#""(.*?)""#).unwrap();

    for steam_install_path in steam_install_paths {
        let library_meta_file = Path::new(&steam_install_path)
            .join("steamapps")
            .join("libraryfolders.vdf");

        if !library_meta_file.exists() {
            continue;
        }

        let file_data = fs::read_to_string(&library_meta_file)
            .map_err(|e| format!("Failed to read library metadata file: {}", e))?;

        let matches: Vec<&str> = re.find_iter(&file_data).map(|m| m.as_str()).collect();

        for i in 0..matches.len() {
            let match_str = matches[i].replace("\"", "");
            if match_str == "path" && i + 1 < matches.len() {
                let lib_path = Path::new(&matches[i + 1].replace("\"", ""))
                    .to_str()
                    .unwrap_or("")
                    .to_string();
                library_folder_paths.push(lib_path.replace("\\\\", "\\"));
            }
        }
    }

    Ok(library_folder_paths)
}
