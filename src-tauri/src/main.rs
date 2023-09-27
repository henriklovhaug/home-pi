// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bus::bus_routes::{Bus, BusTime, BUS};
use chrono::{DateTime, Timelike, Utc};
use chrono_tz::Europe::Paris;
use chrono_tz::Tz;

mod bus;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            say_hello,
            get_bus,
            get_bus_now,
            get_next_n_bus
        ])
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

#[tauri::command]
fn get_bus_now() -> Result<(u8, Vec<u8>), String> {
    let bus = BUS.get_or_init(|| Bus::init());
    let utc: DateTime<Tz> = Utc::now().with_timezone(&Paris);
    let hour = utc.hour();
    Ok(bus.get(
        hour.try_into()
            .map_err(|_| "Failed to get time".to_string())?,
    ))
}

#[tauri::command]
fn get_next_n_bus(n: u32) -> Vec<BusTime> {
    let bus = BUS.get_or_init(|| Bus::init());
    let utc: DateTime<Tz> = Utc::now().with_timezone(&Paris);
    let hour = utc.hour() as u8;
    let minute = utc.minute() as u8;
    let n = n as u8;
    bus.get_next_n(hour, minute, n)
}
