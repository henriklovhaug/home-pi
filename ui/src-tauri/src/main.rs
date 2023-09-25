// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod bus;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![say_hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn say_hello() -> Result<String, String> {
    Ok("Hello from Rust!".to_string())
}
