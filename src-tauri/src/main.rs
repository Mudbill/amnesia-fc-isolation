// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod archive;
mod config;
mod fs;
mod handlers;
mod utils;
mod xml;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handlers::show_window,
            handlers::install_mod
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
