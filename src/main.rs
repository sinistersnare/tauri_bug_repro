#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use polars::prelude::*;
use tauri::Manager;
use tracing::info;

fn main() {
    println!("doin it.");
    let d = df!("x" => &[1,2,3], "y" => &[4,5,6]).expect("Bad DF making.");
    plot(d);
    println!("fuck what?");
}

pub fn plot(d: DataFrame) {
    tracing_subscriber::fmt::init();
    info!("Pushing dataframe: {:?}", d);
    let mut app = tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                app.get_window("main").unwrap().open_devtools();
            }
            app.emit_all("seed", d)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Error building Tauri app.");
    loop {
        // println!("doing it.");
        let iteration = app.run_iteration();
        if iteration.window_count == 0 {
            break;
        }
    }
    println!("duneeeee");
    info!("AGH");
}
