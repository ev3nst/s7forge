use std::fs;

use crate::utils::get_cache_dir::get_cache_dir;

pub fn clear_cache() -> Result<String, String> {
    let cache_dir = get_cache_dir()?;

    if !cache_dir.exists() {
        return Ok("Cache directory does not exist, nothing to clear".to_string());
    }

    let mut cleared_files = Vec::new();
    let mut errors = Vec::new();

    let entries =
        fs::read_dir(&cache_dir).map_err(|e| format!("Failed to read cache directory: {:?}", e))?;

    for entry in entries {
        match entry {
            Ok(file_entry) => {
                let file_path = file_entry.path();
                if file_path.is_file() {
                    match fs::remove_file(&file_path) {
                        Ok(_) => {
                            if let Some(file_name) = file_path.file_name() {
                                cleared_files.push(file_name.to_string_lossy().to_string());
                            }
                        }
                        Err(e) => {
                            errors.push(format!("Failed to remove {}: {}", file_path.display(), e));
                        }
                    }
                }
            }
            Err(e) => {
                errors.push(format!("Failed to read directory entry: {}", e));
            }
        }
    }

    if !errors.is_empty() {
        return Err(format!(
            "Errors occurred while clearing cache: {}",
            errors.join(", ")
        ));
    }

    if cleared_files.is_empty() {
        Ok("Cache directory was already empty".to_string())
    } else {
        Ok(format!(
            "Successfully cleared {} cache files: {}",
            cleared_files.len(),
            cleared_files.join(", ")
        ))
    }
}
