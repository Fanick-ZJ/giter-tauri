use tauri::{Manager, Url, WebviewBuilder};
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf, sync::Mutex, thread};
use crate::{
    core::handle, emit::emit_branch_contribution, types::{
        cache::RepoPath, 
        error::{
            CommandError, CommonErrorCode as CommonError
        },
        fs::Dir, store
    }, utils::{
        dirs,
        fs::{get_first_level_dirs, get_logical_driver},
    }
};
use giter_macros::command_result;
use giter_utils::{
    types::{
        author::Author, branch::Branch, cache::Cache, 
        commit::Commit, diff::ContentDiff, error::GitUtilsErrorCode, 
        file::{ChangedFile, CommittedFile, FileHistoryEntry}, git_data_provider::GitDataProvider, 
        status::WorkStatus
    },
    util::{is_git_repo, set_owner, str_to_oid},
};
use giter_watcher::{
    modify_watcher::ModifyWatcher, error::WatcherErrorCode as WatcherError
};

type DataResult<T> = std::result::Result<T, CommandError<GitUtilsErrorCode>>;
type CommonResult<T> = std::result::Result<T, CommandError<CommonError>>;
type WatcherResult<T> = std::result::Result<T, CommandError<WatcherError>>;



fn get_provider(repo: &str) -> Result<GitDataProvider, GitUtilsErrorCode> {
    let handle = handle::Handle::global();
    let mut provider = GitDataProvider::new(repo)?;
    Ok(provider)
}

fn watch(repo: RepoPath) -> Result<(), WatcherError> {
    let app = handle::Handle::global().app_handle().unwrap();
    let watch_center = app.state::<Mutex<ModifyWatcher>>();
    
    // 修复1: 移除多余的 match 结构
    let mut watcher = watch_center.lock()
    .map_err(|e| WatcherError::Other(format!("Failed to lock watcher center: {:?}", e)))?;

// 修复2: 直接返回转换后的结果
watcher.add_watch(repo)
}

#[tauri::command]
#[command_result]
pub async fn remove_watch(repo: RepoPath) -> CommonResult<()> {
    let app = handle::Handle::global().app_handle().unwrap();
    let watch_center = app.state::<Mutex<ModifyWatcher>>();
    let mut watcher = watch_center.lock()
        .map_err(|_| CommonError::GetWatcherCenterFailed)?;
    let _ = watcher.remove_watch(repo);
    Ok(())
}


#[tauri::command]
#[command_result]
pub async fn repos() -> CommonResult<Vec<store::Repository>> {
    let store = handle::Handle::global().store().unwrap();
    let repos = store.get_repos().map_err(|_| CommonError::GetReposFailed)?;
    Ok(repos)
}

#[tauri::command]
#[command_result]
pub async fn add_watch(repo: RepoPath) -> WatcherResult<()> {
    watch(repo)
}

#[tauri::command]
#[command_result]
pub async fn authors(repo: RepoPath, branch: Branch) -> DataResult<Vec<Author>> {
    let provider = get_provider(&repo)?;
    let authors = provider.authors(&branch);
    authors
}

#[tauri::command]
#[command_result]
pub async fn branches(repo: RepoPath) -> DataResult<Vec<Branch>> {
    let provider = get_provider(&repo)?;
    provider.branches()
}

#[tauri::command]
#[command_result]
pub async fn get_db_path(db: String) -> CommonResult<String> {
    match db.as_str() {
        "store" => Ok(dirs::store_file().unwrap().to_str().unwrap().to_string()),
        "cache" => Ok(dirs::cache_file().unwrap().to_str().unwrap().to_string()),
        "config" => Ok(dirs::config_file().unwrap().to_str().unwrap().to_string()),
        _ => Err(CommonError::DatabaseInvalid(format!("invalid db: {}", db))),
    }
}

#[tauri::command]
pub async fn get_driver() -> CommonResult<Vec<Dir>> {
    let driver = get_logical_driver();
    let mut folders = vec![];
    for item in driver {
        // 舍去最后两个//
        let name = item.chars().take(item.len() - 1).collect();
        folders.push(Dir {
            name,
            path: item,
            is_repo: false,
        });
    }
    Ok(folders)
}

#[tauri::command]
#[command_result]
pub async fn get_folders(path: String) -> CommonResult<Vec<Dir>> {
    let catalog = get_first_level_dirs(&path);
    match catalog {
        Ok(catalog) => {
            let mut folders = vec![];
            for item in catalog.dirs {
                folders.push(Dir {
                    name: item.name,
                    path: item.path,
                    is_repo: item.is_repo,
                });
            }
            Ok(folders)
        }
        Err(err) => Err(CommonError::PathInvalid(err.to_string())),
    }
}

#[tauri::command]
pub async fn get_separator() -> String {
    std::path::MAIN_SEPARATOR.to_string()
}

#[tauri::command]
pub async fn is_repo(repo: RepoPath) -> bool {
    is_git_repo(&repo)
}

#[tauri::command]
#[command_result]
pub async fn work_status(repo: RepoPath) -> DataResult<WorkStatus> {
    let provider = get_provider(&repo)?;
    provider.work_status()
}

#[tauri::command]
#[command_result]
pub async fn set_repo_ownership(repo: RepoPath) -> CommonResult<bool> {
    let provider = get_provider(&repo);
    match provider {
        Ok(_) => Ok(true),
        Err(_) => match set_owner(&repo) {
            Ok(_) => Ok(true),
            Err(err) => {
                let e = GitUtilsErrorCode::Git2Error(err);
                Err(CommonError::SetGlobalConfigError(e.to_string()))
            }
        },
    }
}

#[tauri::command]
#[command_result]
pub async fn branch_commits(repo: RepoPath, branch: Branch, count: i32) -> DataResult<Vec<Commit>> {
    let provider = get_provider(&repo)?;
    provider.branch_commits(&branch, count)
} 

#[tauri::command]
#[command_result]
pub async fn before_reference_commits_count(repo: RepoPath, reference: String) -> DataResult<i32> {
    let provider = get_provider(&repo)?;
    provider.before_reference_commits_count(&reference) 
}

#[tauri::command]
#[command_result]
pub async fn current_branch(repo: RepoPath) -> DataResult<Branch> {
    let provider = get_provider(&repo)?;
    provider.current_branch()
}

#[tauri::command] 
#[command_result]
pub async fn current_remote_branch(repo: RepoPath) -> DataResult<Branch> {
    let provider = get_provider(&repo)?;
    provider.current_remote_branch()
}

#[tauri::command]
#[command_result]
pub async fn commit_content (repo: RepoPath, cid: String) -> DataResult<Vec<CommittedFile>> {
    let provider = get_provider(&repo)?;
    let oid = str_to_oid(&cid)?;
    provider.commit_content(oid)
}

#[tauri::command]
#[command_result]
pub async fn file_diff(repo: RepoPath, old: String, new: String) -> DataResult<ContentDiff> {
    let provider = get_provider(&repo)?;
    let old_id = str_to_oid(&old)?;
    let new_id = str_to_oid(&new)?;
    provider.get_file_content_diff(old_id, new_id)
}

#[tauri::command]
#[command_result]
pub async fn blob_content(repo: RepoPath, cid: String) -> DataResult<Vec<u8>> {
    let provider = get_provider(&repo)?;
    let oid = str_to_oid(&cid)?;
    provider.get_blob_content(oid)
}

#[tauri::command]
#[command_result]
pub async fn get_commit(repo: RepoPath, cid: String) -> DataResult<Commit> {
    let provider = get_provider(&repo)?;
    let commit_id = str_to_oid(&cid)?;
    provider.get_commit(commit_id)
}

#[tauri::command]
#[command_result]
pub async fn get_branch_commit_contribution(key: String, repo: RepoPath, branch: Branch) -> DataResult<()> {
    let provider = get_provider(&repo)?;
    // 由于第一次执行时间很长，所以开一个线程执行
    thread::spawn(move || {
        let contrib = provider.get_branch_commit_contribution(&branch);
        emit_branch_contribution(&key, contrib);
    });
    Ok::<(), GitUtilsErrorCode>(())
}

#[tauri::command]
#[command_result]
pub async fn get_global_author() -> CommonResult<Author> {
    let author = giter_utils::util::get_global_git_author();
    match author {
        Ok(author) => Ok(author),
        Err(e) => Err(CommonError::GetGlobalConfigError(e.to_string())), 
    }
}

#[tauri::command]
#[command_result]
pub async fn get_repo_author(repo: RepoPath) -> DataResult<Author> {
    let provider = get_provider(&repo)?;
    provider.author()
}

#[tauri::command]
#[command_result]
pub async fn reference_commit_filter_details(repo: RepoPath, reference: String, filter: HashMap<String, Value>, offset: Option<i32>, count: Option<i32>) -> DataResult<Vec<Commit>> {
    let provider = get_provider(&repo)?;
    provider.reference_commit_filter_details(&reference, &filter, offset, count)
}

#[tauri::command]
#[command_result]
pub async fn reference_commit_filter_count(repo: RepoPath, reference: String, filter: HashMap<String, Value>, offset: Option<i32>, count: Option<i32>) -> DataResult<i32> {
    let provider = get_provider(&repo)?;
    provider.reference_commit_filter_count(&reference, &filter, offset, count)
}

#[tauri::command]
#[command_result]
pub async fn get_changed_files(repo: RepoPath) -> DataResult<Vec<ChangedFile>> {
    let provider = get_provider(&repo)?;
    provider.changed_files()
}

#[tauri::command]
#[command_result]
pub async fn get_staged_files(repo: RepoPath) -> DataResult<Vec<ChangedFile>> {
    let provider = get_provider(&repo)?;
    provider.staged_files()
}

#[tauri::command]
#[command_result]
pub async fn add_to_stage(repo: RepoPath, path: String) -> DataResult<()> {
    let provider = get_provider(&repo)?;
    provider.add_to_stage(&PathBuf::from(&path))
}

#[tauri::command]
#[command_result]
pub async fn remove_from_stage(repo: RepoPath, path: String) -> DataResult<()> {
    let provider = get_provider(&repo)?;
    provider.remove_from_stage(&PathBuf::from(&path))
}

#[tauri::command]
#[command_result]
pub async fn checkout_file(repo: RepoPath, path: String) -> DataResult<()> {
    let provider = get_provider(&repo)?;
    provider.checkout_file(&PathBuf::from(&path))
}

#[tauri::command]
#[command_result]
pub async fn commit(repo: RepoPath, message: &str, update_ref: Option<&str>) -> DataResult<String> {
    let provider = get_provider(&repo)?;
    let commit_id = provider.commit(message, update_ref)?;
    Ok::<String, GitUtilsErrorCode>(commit_id.to_string())
}

#[tauri::command]
#[command_result]
pub async fn push(repo: RepoPath, remote: String, branch: String, credentials: Option<(String, String)>) -> DataResult<()> {
    let provider = get_provider(&repo)?;
    provider.push(&remote, &branch, credentials)
}
#[tauri::command]
#[command_result]
pub async fn pull(repo: RepoPath, remote: String, branch: String, credentials: Option<(String, String)>) -> DataResult<()> {
    let provider = get_provider(&repo)?;
    provider.pull(&remote, &branch, credentials)
}

#[tauri::command]
#[command_result]
pub async fn switch_branch(repo: RepoPath, branch: Branch) -> DataResult<()> {
    let provider = get_provider(&repo)?;
    provider.switch_branch(&branch) 
}

#[tauri::command]
#[command_result]
pub async fn file_history(repo: RepoPath, file_path: String) -> DataResult<Vec<FileHistoryEntry>> {
    let provider = get_provider(&repo)?;
    provider.file_history(file_path)
}

#[tauri::command]
pub fn create_window(
    app: tauri::AppHandle,
    label: &str,
    title: &str,
    url: &str,
    width: f64,
    height: f64,
    fullscreen: bool,
    resizable: bool
) {
    let windows = app.windows();
    if windows.get(label).is_some() {
        return; 
    }
    let window = tauri::window::WindowBuilder::new(
        &app,
        label, // 必须唯一
        )
        .title(title)
        .inner_size(width, height)
        .fullscreen(fullscreen)
        .resizable(resizable)
        .build()
        .expect("无法创建窗口");
    
    let url = tauri::WebviewUrl::External(Url::parse(url).unwrap());
    let webview = WebviewBuilder::new(label, url);
    window.add_child(webview, tauri::LogicalPosition::new(0, 0), window.inner_size().unwrap());
}