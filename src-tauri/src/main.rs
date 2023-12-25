#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod setup;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(setup::init)
        .run(context)
        .expect("error while running tauri application");


    
  }
