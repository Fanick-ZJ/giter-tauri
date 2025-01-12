mod core;
mod emit;
mod cmd;
mod types;
mod utils;

use crate::utils::resolve;
use cmd::{ add_watch, authors, branches, clear_all_cache, clear_cache };

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                resolve::resolve_setup(app).await;
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, add_watch, authors, branches, clear_all_cache, clear_cache])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
