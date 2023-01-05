#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bridge::MyStruct;
use tauri::Manager;
use tauri_glue::*;

#[tauri_glue::command]
fn hello(name: String, test_struct: MyStruct) -> Result<String, String> {
    Ok(format!(
        "Hello from Tauri, {:?}! {:#?}",
        name, test_struct
    ))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
