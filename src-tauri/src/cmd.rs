use crate::{
    core::handle, emit::emit_branch_contribution, types::{
        cache::RepoPath, 
        error::CommandError as CE,
        fs::Dir, store
    }, utils::{
        dirs,
        fs::{get_first_level_dirs, get_logical_driver},
    }
};
use git2::Oid;
use giter_macros::command_result;
use giter_utils::{
    types::{
        author::Author, branch::Branch, cache::Cache, commit::Commit, diff::ContentDiff, error::ErrorCode, file::{ChangedFile, CommittedFile}, git_data_provider::GitDataProvider, status::WorkStatus
    },
    util::{is_git_repo, set_owner},
};
use giter_watcher::types::modify_watcher::ModifyWatcher;
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf, sync::Mutex, thread};
use tauri::Manager;

fn str_to_oid(str: &str) -> Result<Oid, ErrorCode> {
    let oid = Oid::from_str(str);
    match oid {
        Ok(oid) => Ok(oid),
        Err(e) => Err(ErrorCode::OtherError(format!("invalid object id: {}", str))), 
    }
}

fn get_provider(repo: &str) -> Result<GitDataProvider, ErrorCode> {
    let handle = handle::Handle::global();
    let mut provider = GitDataProvider::new(repo)?;
    let cache = handle.cache().unwrap();
    provider.set_cache(cache);
    Ok(provider)
}

fn watch(repo: RepoPath) -> Result<(), CE> {
    let app = handle::Handle::global().app_handle().unwrap();
    let watch_center = app.state::<Mutex<ModifyWatcher>>();
    let watcher = watch_center.lock();
    match watcher {
        Ok(mut watcher) => {
            watcher.add_watch(repo);
            Ok(())
        },
        Err(e) => Err(CE {
            code: 90,   // 暂时定为90，后续再定义
            message: e.to_string(),
            func: stringify!(watch).to_string(),
            data: Some(vec![repo.to_string()]),
        }), 
    }
}

#[tauri::command]
pub fn remove_watch(repo: RepoPath) -> Result<(), CE> {
    let app = handle::Handle::global().app_handle().unwrap();
    let watch_center = app.state::<Mutex<ModifyWatcher>>();
    let watcher = watch_center.lock();
    match watcher {
        Ok(mut watcher) => {
            watcher.remove_watch(repo);
            Ok(())
        },
        Err(e) => Err(CE {
            code: 90,   // 暂时定为90，后续再定义
            message: e.to_string(),
            func: stringify!(remove_watch).to_string(),
            data: Some(vec![repo.to_string()]),
        }), 
    }
}


#[tauri::command]
pub fn repos() -> Result<Vec<store::Repository>, CE> {
    let store = handle::Handle::global().store().unwrap();
    let repos = store.get_repos();
    match repos {
        Ok(repos) => Ok(repos),
        Err(e) => Err(CE {
            code: 90,   // 暂时定为90，后续再定义
            message: e.to_string(),
            func: stringify!(repos).to_string(),
            data: None,
        }),
    }
}

#[tauri::command]
pub fn add_watch(repo: RepoPath) -> Result<(), CE> {
    watch(repo)
}

#[tauri::command]
#[command_result]
pub fn authors(repo: RepoPath, branch: Branch) -> Result<Vec<Author>, CE> {
    let provider = get_provider(&repo)?;
    let authors = provider.authors(&branch);
    authors
}

#[tauri::command]
#[command_result]
pub fn branches(repo: RepoPath) -> Result<Vec<Branch>, CE> {
    let provider = get_provider(&repo)?;
    provider.branches()
}
// #[tauri::command]
// // #[command_result(CEE::BranchesFindError)]
// pub fn branches(repo: RepoPath) -> Result<Vec<Branch>, CE> {
//     let provider = get_provider(&repo)?;
//     let branches = provider.branches();
//     match branches {
//         Ok(branches) => Ok(branches),
//         Err(e) => Err(CE {
//             code: e.code(),
//             message: e.to_string(),
//             func: stringify!(branches).to_string(),
//             data: Some(vec![repo.to_string()]),
//         }), 
//     }
// }

#[tauri::command]
pub fn clear_cache(repo: RepoPath) {
    let mut cache = handle::Handle::global().cache().unwrap();
    cache.clear(&repo);
}

#[tauri::command]
pub fn clear_all_cache() {
    let mut cache = handle::Handle::global().cache().unwrap();
    cache.clear_all();
}

#[tauri::command]
pub fn get_db_path(db: String) -> Result<String, CE> {
    match db.as_str() {
        "store" => Ok(dirs::store_file().unwrap().to_str().unwrap().to_string()),
        "cache" => Ok(dirs::cache_file().unwrap().to_str().unwrap().to_string()),
        "config" => Ok(dirs::config_file().unwrap().to_str().unwrap().to_string()),
        _ => Err(CE {
            code: 90,   // 暂时定为90，后续再定义
            message: "invalid db".to_string(),
            func: stringify!(get_db_path).to_string(),
            data: Some(vec![db]),
        }),
    }
}

#[tauri::command]
pub fn get_driver() -> Result<Vec<Dir>, CE> {
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
pub fn get_folders(path: String) -> Result<Vec<Dir>, CE> {
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
        Err(e) => Err(CE {
            code: 90,   // 暂时定为90，后续再定义
            message: e.to_string(),
            func: stringify!(get_folders).to_string(),
            data: Some(vec![path]),
        }),
    }
}

#[tauri::command]
pub fn get_separator() -> String {
    std::path::MAIN_SEPARATOR.to_string()
}

#[tauri::command]
pub async fn is_repo(repo: RepoPath) -> bool {
    is_git_repo(&repo)
}

#[tauri::command]
#[command_result]
pub async fn work_status(repo: RepoPath) -> Result<WorkStatus, CE> {
    let provider = get_provider(&repo)?;
    provider.work_status()
}

#[tauri::command]
pub fn set_repo_ownership(repo: RepoPath) -> Result<bool, CE> {
    let provider = get_provider(&repo);
    match provider {
        Ok(_) => Ok(true),
        Err(_) => match set_owner(&repo) {
            Ok(_) => Ok(true),
            Err(err) => {
                let e = ErrorCode::Git2Error(err);
                Err(CE {
                    code: e.code(),
                    message: e.to_string(),
                    func: stringify!(set_repo_ownership).to_string(),
                    data: Some(vec![repo.to_string()]),
                })
            }
        },
    }
}

#[tauri::command]
#[command_result]
pub fn branch_commits(repo: RepoPath, branch: Branch, count: i32) -> Result<Vec<Commit>, CE> {
    let provider = get_provider(&repo)?;
    provider.get_branch_commits(&branch, count)
} 

#[tauri::command]
#[command_result]
pub fn current_branch(repo: RepoPath) -> Result<Branch, CE> {
    let provider = get_provider(&repo)?;
    provider.current_branch()
}

#[tauri::command] 
#[command_result]
pub fn current_remote_branch(repo: RepoPath) -> Result<Branch, CE> {
    let provider = get_provider(&repo)?;
    provider.current_remote_branch()
}

#[tauri::command]
#[command_result]
pub fn commit_content (repo: RepoPath, cid: String) -> Result<Vec<CommittedFile>, CE> {
    let provider = get_provider(&repo)?;
    let oid = str_to_oid(&cid)?;
    provider.commit_content(oid)
}

#[tauri::command]
#[command_result]
pub fn file_diff(repo: RepoPath, old: String, new: String) -> Result<ContentDiff, CE> {
    let provider = get_provider(&repo)?;
    let old_id = str_to_oid(&old)?;
    let new_id = str_to_oid(&new)?;
    provider.get_file_content_diff(old_id, new_id)
}

#[tauri::command]
#[command_result]
pub fn blob_content(repo: RepoPath, cid: String) -> Result<Vec<u8>, CE> {
    let provider = get_provider(&repo)?;
    let oid = str_to_oid(&cid)?;
    provider.get_blob_content(oid)
}

#[tauri::command]
#[command_result]
pub fn get_commit(repo: RepoPath, cid: String) -> Result<Commit, CE> {
    let provider = get_provider(&repo)?;
    let commit_id = str_to_oid(&cid)?;
    provider.get_commit(commit_id)
}

#[tauri::command]
#[command_result]
pub fn get_branch_commit_contribution(key: String, repo: RepoPath, branch: Branch) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    // 由于第一次执行时间很长，所以开一个线程执行
    thread::spawn(move || {
        let contrib = provider.get_branch_commit_contribution(&branch);
        emit_branch_contribution(&key, contrib);
    });
    Ok(())
}

#[tauri::command]
pub fn get_global_author() -> Result<Author, CE> {
    let author = giter_utils::util::get_global_git_author();
    match author {
        Ok(author) => Ok(author),
        Err(e) => Err(CE {
            code: 90,   // 暂时定为90，后续再定义
            message: e.to_string(),
            func: stringify!(get_global_author).to_string(),
            data: None,
        }), 
    }
}

#[tauri::command]
#[command_result]
pub fn get_repo_author(repo: RepoPath) -> Result<Author, CE> {
    let provider = get_provider(&repo)?;
    provider.author()
}

#[tauri::command]
#[command_result]
pub fn get_branch_commits_after_filter(repo: RepoPath, branch: Branch, filter: HashMap<String, Value>) -> Result<Vec<Commit>, CE> {
    let provider = get_provider(&repo)?;
    provider.get_branch_commits_after_filter(&branch, &filter)
}

#[tauri::command]
#[command_result]
pub fn get_changed_files(repo: RepoPath) -> Result<Vec<ChangedFile>, CE> {
    let provider = get_provider(&repo)?;
    provider.changed_files()
}

#[tauri::command]
#[command_result]
pub fn get_staged_files(repo: RepoPath) -> Result<Vec<ChangedFile>, CE> {
    let provider = get_provider(&repo)?;
    provider.staged_files()
}

#[tauri::command]
#[command_result]
pub fn add_to_stage(repo: RepoPath, path: String) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    provider.add_to_stage(&PathBuf::from(&path))
}

#[tauri::command]
#[command_result]
pub fn remove_from_stage(repo: RepoPath, path: String) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    provider.remove_from_stage(&PathBuf::from(&path))
}

#[tauri::command]
#[command_result]
pub fn checkout_file(repo: RepoPath, path: String) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    provider.checkout_file(&PathBuf::from(&path))
}

#[tauri::command]
#[command_result]
pub fn commit(repo: RepoPath, message: &str, update_ref: Option<&str>) -> Result<String, CE> {
    let provider = get_provider(&repo)?;
    let commit_id = provider.commit(message, update_ref)?;
    Ok(commit_id.to_string())
}

#[tauri::command]
#[command_result]
pub fn push(repo: RepoPath, remote: String, branch: String, credentials: Option<(String, String)>) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    provider.push(&remote, &branch, credentials)
}
#[tauri::command]
#[command_result]
pub fn pull(repo: RepoPath, remote: String, branch: String, credentials: Option<(String, String)>) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    provider.pull(&remote, &branch, credentials)
}