mod cmd;
mod core;
mod emit;
mod types;
mod utils;

use std::collections::HashMap;

use crate::{
    cmd::{get_commit_tree_recursive, get_repo_by_path, get_tree, object_is_binary, save_blob},
    utils::resolve,
};
use cmd::{
    add_to_stage, add_watch, authors, before_reference_commits_count, blob_content, branch_commits,
    branches, checkout_file, commit, commit_content, create_window, current_branch,
    current_remote_branch, file_diff, file_history, get_branch_commit_contribution,
    get_changed_files, get_commit, get_db_path, get_driver, get_folders, get_global_author,
    get_repo_author, get_separator, get_staged_files, is_repo, pull, push,
    reference_commit_filter_count, reference_commit_filter_details, remove_from_stage,
    remove_watch, repos, set_repo_ownership, switch_branch, work_status,
};
use parking_lot::RwLock;
use types::cache::RepoPath;
pub struct SingleRepoSubmit(RwLock<HashMap<String, i32>>);

#[tauri::command]
fn repo_single_submit(repo: RepoPath, state: tauri::State<SingleRepoSubmit>) {
    let mut map = state.inner().0.write();
    map.entry(repo).and_modify(|v| *v += 1).or_insert(0);
}

#[tauri::command]
fn repo_single_unsubmit(repo: RepoPath, state: tauri::State<SingleRepoSubmit>) {
    let mut map = state.inner().0.write();
    if let std::collections::hash_map::Entry::Occupied(mut e) = map.entry(repo) {
        let value = e.get_mut();
        *value -= 1;
        if *value == 0 {
            e.remove();
        }
    }
}

#[cfg(debug_assertions)]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    use tauri_plugin_prevent_default::Flags;

    tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::all().difference(Flags::DEV_TOOLS | Flags::RELOAD))
        .build()
}

#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::init()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(SingleRepoSubmit(RwLock::new(HashMap::new())))
        .plugin(prevent_default())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let _ = resolve::init_tray(app).await;
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
            get_db_path,
            get_driver,
            get_folders,
            get_separator,
            is_repo,
            work_status,
            set_repo_ownership,
            remove_watch,
            branch_commits,
            before_reference_commits_count,
            reference_commit_filter_details,
            reference_commit_filter_count,
            current_branch,
            commit_content,
            file_diff,
            blob_content,
            get_commit,
            get_branch_commit_contribution,
            get_global_author,
            get_repo_author,
            reference_commit_filter_details,
            get_changed_files,
            get_staged_files,
            add_to_stage,
            remove_from_stage,
            checkout_file,
            commit,
            current_remote_branch,
            push,
            pull,
            switch_branch,
            repo_single_submit,
            repo_single_unsubmit,
            file_history,
            create_window,
            get_commit_tree_recursive,
            get_tree,
            object_is_binary,
            get_repo_by_path,
            save_blob
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
