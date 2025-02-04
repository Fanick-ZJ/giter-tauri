mod cmd;
mod core;
mod emit;
mod types;
mod utils;

use crate::utils::resolve;
use cmd::{
    add_watch, authors, branch_commits, branches, clear_all_cache, clear_cache, commit_content, current_branch, file_diff, get_db_path, get_driver, get_folders, get_separator, is_repo, remove_watch, repos, set_repo_ownership, work_status
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let _ = resolve::resolve_setup(app).await;
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            add_watch,
            repos,
            authors,
            branches,
            clear_all_cache,
            clear_cache,
            get_db_path,
            get_driver,
            get_folders,
            get_separator,
            is_repo,
            work_status,
            set_repo_ownership,
            remove_watch,
            branch_commits,
            current_branch,
            commit_content,
            file_diff,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
