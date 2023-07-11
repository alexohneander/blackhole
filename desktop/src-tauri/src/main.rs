// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod security;
mod helpers;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn upload_file(path: &str) -> String {
    println!("Backend Side File: {}", &path);

    // File Encryption
    let result = security::encryption::encrypt_file(&path);
    match &result {
        Ok(_) => println!("File encrypted successfully"),
        Err(e) => println!("Error encrypting file: {}", e),
    }

    //TODO: File Upload

    return result.unwrap().enc_token;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![upload_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
