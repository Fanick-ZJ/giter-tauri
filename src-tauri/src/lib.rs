mod core;
mod emit;
mod types;
mod utils;

use crate::utils::resolve;
use giter_utils::types::git_data_provider::GitDataProvider;
use giter_watcher::types::modify_watcher::ModifyWatcher;
use imara_diff;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::Manager;

use types::error::CommandError;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[tauri::command]
    fn add_watch(
        path: String,
        watcher_center: tauri::State<'_, Mutex<ModifyWatcher>>,
        data_providers: tauri::State<'_, Mutex<HashMap<String, GitDataProvider>>>,
    ) -> Result<(), CommandError> {
        match (watcher_center.lock(), data_providers.lock()) {
            (Ok(mut watcher), Ok(mut providers)) => {
                // 判断是否已经加载过了
                if let None = providers.get(&path) {
                    let provider = GitDataProvider::new(&path);
                    if let Ok(provider) = provider {
                        providers.insert(path.clone(), provider);
                        watcher.add_watch(path);
                    } else {
                        // 非法路径
                        return Err(CommandError::InvalidRepository(path));
                    }
                } else {
                    return Err(CommandError::RepositoryHasWatched(path));
                }
            }
            _ => {}
        }
        Ok(())
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
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
