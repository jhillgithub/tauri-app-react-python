// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn list_files(path: String) -> Result<String, String> {
    let output = Command::new("python")
        .arg("src/listFiles.py")
        .arg(&path)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !stderr.is_empty() {
        Err(format!("Error listing files: {}", stderr))
    } else {
        Ok(stdout)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
