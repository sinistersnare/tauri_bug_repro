#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub fn main() {
    let mut app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("Error building Tauri app.");
    loop {
        let iteration = app.run_iteration();
        if iteration.window_count == 0 {
            break;
        }
    }
}
