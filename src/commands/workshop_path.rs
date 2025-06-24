use std::fs;
use std::path::Path;

use crate::core::steam_install_paths::steam_install_paths;
use crate::utils::extract_quoted_strings::extract_quoted_strings;

pub fn workshop_path(app_id: u32) -> Option<String> {
    match steam_install_paths() {
        Ok(paths) => {
            for steam_install_path in paths {
                let library_meta_file = Path::new(&steam_install_path)
                    .join("steamapps")
                    .join("libraryfolders.vdf");

                if !library_meta_file.exists() {
                    continue;
                }

                let file_data = match fs::read_to_string(&library_meta_file) {
                    Ok(data) => data,
                    Err(_) => continue,
                };

                let quoted_strings = extract_quoted_strings(&file_data);

                let mut library_folder_paths = Vec::new();
                for i in 0..quoted_strings.len() {
                    let current_string = &quoted_strings[i];
                    if current_string == "path" && i + 1 < quoted_strings.len() {
                        let lib_path = Path::new(&quoted_strings[i + 1])
                            .to_str()
                            .unwrap_or("")
                            .to_string();
                        library_folder_paths.push(lib_path.replace("\\\\", "\\"));
                    }
                }

                for lib_path in &library_folder_paths {
                    let workshop_path = Path::new(lib_path)
                        .join("steamapps")
                        .join("workshop")
                        .join("content")
                        .join(app_id.to_string());

                    if workshop_path.exists() {
                        return Some(workshop_path.to_string_lossy().into_owned());
                    }
                }
            }
            None
        }
        Err(_) => None,
    }
}
