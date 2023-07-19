// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actix_web::rt::Runtime;
use server::serve::serve;

mod server;

fn main() {
    let ctx = tauri::generate_context!();

    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(serve());
    });

    tauri::Builder::default()
        .run(ctx)
        .expect("error while running tauri application");
}
