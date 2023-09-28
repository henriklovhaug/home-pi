// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bus::bus_routes::{Bus, BusTime, BUS};
use chrono::{Timelike, Utc};
use chrono_tz::Europe::Paris;
use weather::{api::get_weather_backend, types::Weather};

mod bus;
mod weather;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            say_hello,
            get_next_n_bus,
            get_weather,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn say_hello() -> Result<String, String> {
    Ok("Hello from Rust!".to_string())
}

#[tauri::command]
fn get_next_n_bus(n: u32) -> Vec<BusTime> {
    let bus = BUS.get_or_init(Bus::init);
    let time = Utc::now().with_timezone(&Paris);
    let hour = time.hour() as u8;
    let minute = time.minute() as u8;
    let n = n as u8;
    bus.get_next_n(hour, minute, n)
}

#[tauri::command]
async fn get_weather() -> Result<Weather, String> {
    get_weather_backend().await
}
