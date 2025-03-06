use crate::{
    core::handle, 
    emit::emit_branch_contribution, 
    types::{
        cache::RepoPath, 
        error::{
            CommandErrorEnum as CEE,
            CommandError as CE,
        },
        fs::Dir, store
    }, 
    utils::{
        dirs,
        fs::{get_first_level_dirs, get_logical_driver},
    }
};
use git2::Oid;
use giter_utils::{
    types::{
        author::Author, branch::Branch, cache::Cache, commit::Commit,  diff::ContentDiff, file::{ChangedFile, CommittedFile}, git_data_provider::GitDataProvider, status::WorkStatus
    },
    util::{is_git_repo, set_owner},
};
use giter_watcher::types::modify_watcher::ModifyWatcher;
use serde_json::Value;
use std::{collections::HashMap, path::PathBuf, sync::Mutex, thread};
use tauri::Manager;

fn get_provider(repo: &str) -> Result<GitDataProvider, CE> {
    let handle = handle::Handle::global();
    let provider = GitDataProvider::new(repo);
    match provider {
        Ok(mut provider) => {
            let cache = handle.cache().unwrap();
            provider.set_cache(cache);
            Ok(provider)
        }
        Err(err) => match err.code() {
            git2::ErrorCode::Owner => Err(CE {
                message: err.message().to_string(),
                code: CEE::RepoHasnotOwnership,
                data: Some(vec![repo.to_string()]),
            }),
            _ => Err(CE {
                message: err.message().to_string(),
                code: CEE::DataProviderBuildError,
                data: Some(vec![repo.to_string()]),
            }),
        },
    }
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
            message: e.to_string(),
            code: CEE::AddWatcherError,
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
            message: e.to_string(),
            code: CEE::RemoveWatcherError,
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
            message: e.to_string(),
            code: CEE::AddRepositoryStoreError,
            data: None,
        }),
    }
}

#[tauri::command]
pub fn add_watch(repo: RepoPath) -> Result<(), CE> {
    watch(repo)
}

#[tauri::command]
pub fn authors(repo: RepoPath, branch: Branch) -> Result<Vec<Author>, CE> {
    let provider = get_provider(&repo)?;
    let authors = provider.authors(&branch);
    if let Err(_) = authors {
        return Err(CE {
            message: "get authors error".to_string(),
            code: CEE::GetAuthorError,
            data: Some(vec![repo.to_string(), branch.name]),
        });
    }
    Ok(authors.unwrap())
}

#[tauri::command]
pub fn branches(repo: RepoPath) -> Result<Vec<Branch>, CE> {
    let provider = get_provider(&repo)?;
    let branches = provider.branches();
    match branches {
        Ok(branches) => Ok(branches),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::BranchesFindError,
            data: Some(vec![repo.to_string()]),
        }), 
    }
}

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
        e => Err(CE {
            message: "invalid db".to_string(),
            code: CEE::DbNotFound,
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
            message: e.to_string(),
            code: CEE::GetFoldersError,
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
pub async fn work_status(repo: RepoPath) -> Result<WorkStatus, CE> {
    let provider = get_provider(&repo)?;
    let statuses = provider.work_status();
    match statuses {
        Ok(statuses) => Ok(statuses),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::GetWorkStatusError,
            data: Some(vec![repo.to_string()]),
        }), 
    }
}

#[tauri::command]
pub fn set_repo_ownership(repo: RepoPath) -> Result<bool, CE> {
    let provider = get_provider(&repo);
    match provider {
        Ok(_) => Ok(true),
        Err(_) => match set_owner(&repo) {
            Ok(_) => Ok(true),
            Err(err) => Err(CE {
                message: err.to_string(),
                code: CEE::SetRepoOwnershipError,
                data: Some(vec![repo.to_string()]),
            }),
        },
    }
}

#[tauri::command]
pub fn branch_commits(repo: RepoPath, branch: Branch, count: i32) -> Result<Vec<Commit>, CE> {
    let provider = get_provider(&repo)?;
    let commits = provider.get_branch_commits(&branch, count);
    match commits {
        Ok(commits) => Ok(commits),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::GetBranchCommitsError,
            data: Some(vec![repo.to_string(), branch.name]),
        }), 
    }
} 

#[tauri::command]
pub fn current_branch(repo: RepoPath) -> Result<Branch, CE> {
    let provider = get_provider(&repo)?;
    let branch = provider.current_branch();
    match branch {
        Ok(branch) => Ok(branch),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::GetCurrentBranchError,
            data: Some(vec![repo.to_string()]),
        }), 
    }
}

#[tauri::command] 
pub fn current_remote_branch(repo: RepoPath) -> Result<Branch, CE> {
    let provider = get_provider(&repo)?;
    let branch = provider.current_remote_branch();
    match branch {
        Ok(branch) => Ok(branch),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::GetCurrentRemoteBranchError,
            data: Some(vec![repo.to_string()]),
        }), 
    }
}

#[tauri::command]
pub fn commit_content (repo: RepoPath, cid: String) -> Result<Vec<CommittedFile>, CE> {
    let provider = get_provider(&repo)?;
    let oid = Oid::from_str(&cid);
    if let Err(e) = oid {
        return Err(CE{
            message: e.to_string(),
            code: CEE::ConvertOidError,
            data: Some(vec![repo.to_string()]),
        });
    }
    let commit_id = oid.unwrap();
    let content = provider.commit_content(commit_id);
    match content {
        Ok(content) => Ok(content),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::GetCommitContentError,
            data: Some(vec![repo.to_string()]),
        }), 
    }
}

#[tauri::command]
pub fn file_diff(repo: RepoPath, old: String, new: String) -> Result<ContentDiff, CE> {
    let provider = get_provider(&repo)?;
    let old_id = Oid::from_str(&old);
    if let Err(e) = old_id {
        return Err(CE { message: e.to_string(), code: CEE::ConvertOidError, data: Some(vec![repo.to_string(), old, new]) });
    }
    let old_id = old_id.unwrap();
    let new_id = Oid::from_str(&new);
    if let Err(e) = new_id {
        return Err(CE { message: e.to_string(), code: CEE::ConvertOidError, data: Some(vec![repo.to_string(), old, new]) });
    }
    let new_id = new_id.unwrap();
    let diff = provider.get_file_content_diff(old_id, new_id);
    if let Err(e) = diff {
        return Err(CE {
            message: e.to_string(),
            code: CEE::GetFileDiffError,
            data: Some(vec![repo.to_string(), old, new]),
        });
    }
    Ok(diff.unwrap())
}

#[tauri::command]
pub fn blob_content(repo: RepoPath, cid: String) -> Result<Vec<u8>, CE> {
    let provider = get_provider(&repo)?;
    let oid = Oid::from_str(&cid);
    if let Err(e) = oid {
        return Err(CE {
            message: e.to_string(),
            code: CEE::ConvertOidError,
            data: Some(vec![repo.to_string(), cid]),
        });
    }
    let commit_id = oid.unwrap();
    let content = provider.get_blob_content(commit_id);
    match content {
        Ok(content) => Ok(content),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::GetFileContentError,
            data: Some(vec![repo.to_string(), cid]),
        }), 
    }
}

#[tauri::command]
pub fn get_commit(repo: RepoPath, cid: String) -> Result<Commit, CE> {
    let provider = get_provider(&repo)?;
    let oid = Oid::from_str(&cid);
    if let Err(e) = oid {
        return Err(CE {
            message: e.to_string(),
            code: CEE::ConvertOidError,
            data: Some(vec![repo.to_string(), cid]),
        });
    }
    let commit_id = oid.unwrap();
    let commit = provider.get_commit(commit_id);
    if let Err(e) = commit {
        return Err(CE {
            message: e.to_string(),
            code: CEE::GetCommitError,
            data: Some(vec![repo.to_string(), cid]),
        });
    }
    Ok(commit.unwrap())
}

#[tauri::command]
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
            message: e.to_string(),
            code: CEE::GetGlobalAuthorError,
            data: None,
        }), 
    }
}

#[tauri::command]
pub fn get_repo_author(repo: RepoPath) -> Result<Author, CE> {
    let provider = get_provider(&repo)?;
    let author = provider.author();
    if let Err(e) = author {
        return Err(CE {
            message: e.to_string(),
            code: CEE::GetRepoAuthorError,
            data: Some(vec![repo.to_string()]),
        });
    }
    Ok(author.unwrap())
}

#[tauri::command]
pub fn get_branch_commits_after_filter(repo: RepoPath, branch: Branch, filter: HashMap<String, Value>) -> Result<Vec<Commit>, CE> {
    let provider = get_provider(&repo)?;
    let commits = provider.get_branch_commits_after_filter(&branch, &filter);
    if let Err(e) = commits {
        return Err(CE {
            message: e.to_string(),
            code: CEE::GetBranchCommitsError,
            data: Some(vec![repo.to_string(), branch.name]),
        });
    }
    Ok(commits.unwrap())
}

#[tauri::command]
pub fn get_changed_files(repo: RepoPath) -> Result<Vec<ChangedFile>, CE> {
    let provider = get_provider(&repo)?;
    let files = provider.changed_files();
    match files {
        Ok(files) => Ok(files),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::GetChangedFilesError,
            data: Some(vec![repo.to_string()]),
        }), 
    }
}

#[tauri::command]
pub fn get_staged_files(repo: RepoPath) -> Result<Vec<ChangedFile>, CE> {
    let provider = get_provider(&repo)?;
    let files = provider.staged_files();
    if let Err(e) = files {
        return Err(CE {
            message: e.to_string(),
            code: CEE::GetStagedFilesError,
            data: Some(vec![repo.to_string()]),
        });
    }
    Ok(files.unwrap())
}

#[tauri::command]
pub fn add_to_stage(repo: RepoPath, path: String) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    let result = provider.add_to_stage(&PathBuf::from(&path)); 
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::AddToStageError,
            data: Some(vec![repo.to_string(), path]),
        }), 
    }
}

#[tauri::command]
pub fn remove_from_stage(repo: RepoPath, path: String) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    let result = provider.remove_from_stage(&PathBuf::from(&path));
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::RemoveFromStageError,
            data: Some(vec![repo.to_string(), path]),
        }), 
    }
}

#[tauri::command]
pub fn checkout_file(repo: RepoPath, path: String) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    let result = provider.checkout_file(&PathBuf::from(&path));
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::CheckoutFileError,
            data: Some(vec![repo.to_string(), path]),
        }), 
    }
}

#[tauri::command]
pub fn commit(repo: RepoPath, message: &str, update_ref: Option<&str>) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    let result = provider.commit(message, update_ref);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::CommitError,
            data: Some(vec![repo.to_string(), message.to_string()]),
        }), 
    }
}

#[tauri::command]
pub fn push(repo: RepoPath, remote: String, branch: String, credentials: Option<(String, String)>) -> Result<(), CE> {
    let provider = get_provider(&repo)?;
    let result = provider.push(&remote, &branch, credentials);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(CE {
            message: e.to_string(),
            code: CEE::PushError,
            data: Some(vec![repo.to_string(), remote, branch]),
        }), 
    }
}