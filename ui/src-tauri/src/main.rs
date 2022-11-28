#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn my_func(string: &str) {
    println!("{}", string);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_func])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}