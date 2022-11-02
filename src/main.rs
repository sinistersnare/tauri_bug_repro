#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[allow(dead_code)]
fn not_working<A: tauri::Assets>(ctx: tauri::Context<A>) -> Result<(), tauri::Error> {
    let mut app = tauri::Builder::default()
        .build(ctx)
        .expect("Error building Tauri app.");
    tauri::WindowBuilder::new(&app, "main", tauri::WindowUrl::App("index.html".into())).build()?;
    loop {
        let iteration = app.run_iteration();
        if iteration.window_count == 0 {
            break;
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn working<A: tauri::Assets>(ctx: tauri::Context<A>) -> Result<(), tauri::Error> {
    let mut app = tauri::Builder::default()
        .build(ctx)
        .expect("Error building Tauri app.");
    app.run_iteration(); // <<------- KEY LINE TO AVOID RACE
    tauri::WindowBuilder::new(&app, "main", tauri::WindowUrl::App("index.html".into())).build()?;
    loop {
        let iteration = app.run_iteration();
        if iteration.window_count == 0 {
            break;
        }
    }
    Ok(())
}

pub fn main() -> Result<(), tauri::Error> {
    let ctx = tauri::generate_context!();
    // not_working(ctx)?;
    working(ctx)?;
    Ok(())
}
