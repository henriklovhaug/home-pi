// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bus::bus_routes::{Bus, BUS};

mod bus;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![say_hello, get_bus])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn say_hello() -> Result<String, String> {
    Ok("Hello from Rust!".to_string())
}

#[tauri::command]
fn get_bus(hour: u8) -> Result<(u8, Vec<u8>), String> {
    let bus = BUS.get_or_init(|| Bus::init());
    Ok(bus.get(hour))
}
