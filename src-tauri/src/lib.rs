use std::path::{Path, PathBuf};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Expand a leading `~` or `~/…` to the user's home directory.
fn expand_tilde(input: &str) -> PathBuf {
    let trimmed = input.trim();
    if trimmed == "~" {
        return dirs_home();
    }
    if let Some(rest) = trimmed.strip_prefix("~/") {
        let mut home = dirs_home();
        home.push(rest);
        return home;
    }
    PathBuf::from(trimmed)
}

fn dirs_home() -> PathBuf {
    // std::env::home_dir is deprecated but fine for our desktop-only use case;
    // prefer $HOME on unix and %USERPROFILE% on windows.
    #[cfg(windows)]
    {
        if let Ok(p) = std::env::var("USERPROFILE") {
            return PathBuf::from(p);
        }
    }
    #[cfg(not(windows))]
    {
        if let Ok(p) = std::env::var("HOME") {
            return PathBuf::from(p);
        }
    }
    PathBuf::from(".")
}

#[tauri::command]
fn save_image_to_folder(
    folder: String,
    filename: String,
    bytes: Vec<u8>,
) -> Result<String, String> {
    if folder.trim().is_empty() {
        return Err("Output folder is empty.".to_string());
    }
    if filename.trim().is_empty() {
        return Err("Filename is empty.".to_string());
    }
    // Disallow path separators in the filename to keep writes inside `folder`.
    if filename.contains('/') || filename.contains('\\') {
        return Err("Filename must not contain path separators.".to_string());
    }

    let folder_path = expand_tilde(&folder);

    if !folder_path.exists() {
        std::fs::create_dir_all(&folder_path)
            .map_err(|e| format!("Could not create folder: {}", e))?;
    } else if !folder_path.is_dir() {
        return Err(format!(
            "Path exists but is not a directory: {}",
            folder_path.display()
        ));
    }

    let full_path: PathBuf = Path::new(&folder_path).join(&filename);
    std::fs::write(&full_path, &bytes).map_err(|e| format!("Could not write file: {}", e))?;

    Ok(full_path.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet, save_image_to_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
