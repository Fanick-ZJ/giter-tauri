mod utils;
mod core;
mod emit;
mod types;

use std::collections::HashMap;
use crate::utils::resolve;
use std::sync::Mutex;
use tauri::{ Manager};
use giter_utils::types::git_data_provider::GitDataProvider;
use giter_watcher::types::modify_watcher::ModifyWatcher;

use types::{ error::CommandError };

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    #[tauri::command]
    fn add_watch(path: String,
                 watcher_center: tauri::State<'_, Mutex<ModifyWatcher>>,
                 data_providers: tauri::State<'_, Mutex<HashMap<String, GitDataProvider>>>)
    -> Result<(), CommandError>
    {
        match (watcher_center.lock(), data_providers.lock()) {
            (Ok(mut watcher), Ok(mut providers)) => {
                // 判断是否已经加载过了
                match providers.get(&path) {
                    None => {
                        providers.insert(path.clone(), GitDataProvider::new(&path));
                        watcher.add_watch(path);
                    },
                    _ => {
                        return Err(CommandError::RepositoryHasWatched(path))
                    }
                }
            },
            _ => {}
        }
        Ok(())
    }


    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                resolve::resolve_setup(app).await;
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, add_watch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
