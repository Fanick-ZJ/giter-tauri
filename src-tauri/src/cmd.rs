use crate::{
    core::handle, emit::emit_branch_contribution, types::{cache::RepoPath, error::CommandError, fs::Dir, store}, utils::{
        dirs,
        fs::{get_first_level_dirs, get_logical_driver},
    }
};
use git2::Oid;
use giter_utils::{
    types::{
        author::Author, branch::Branch, cache::Cache, commit::Commit, contribution::CommitStatistic, diff::ContentDiff, file::File, git_data_provider::GitDataProvider, status::WorkStatus
    },
    util::{is_git_repo, set_owner},
};
use giter_watcher::types::modify_watcher::ModifyWatcher;
use serde_json::Value;
use std::{collections::HashMap, sync::Mutex, thread};
use tauri::Manager;

fn get_provider(repo: &str) -> Result<GitDataProvider, CommandError> {
    let handle = handle::Handle::global();
    let provider = GitDataProvider::new(repo);
    match provider {
        Ok(mut provider) => {
            let cache = handle.cache().unwrap();
            provider.set_cache(cache);
            Ok(provider)
        }
        Err(err) => match err.code() {
            git2::ErrorCode::Owner => Err(CommandError::RepoHasnotOwnership(repo.to_string())),
            _ => Err(CommandError::DataProviderBuildError(repo.to_string())),
        },
    }
}

fn watch(repo: RepoPath) -> Result<(), CommandError> {
    let app = handle::Handle::global().app_handle().unwrap();
    let watch_center = app.state::<Mutex<ModifyWatcher>>();
    let watcher = watch_center.lock();
    if let Ok(mut watcher) = watcher {
        watcher.add_watch(repo);
        Ok(())
    } else {
        log::error!("watcher center is not ready");
        Err(CommandError::AddWatcherError(repo))
    }
}

#[tauri::command]
pub fn remove_watch(repo: RepoPath) -> Result<(), CommandError> {
    let app = handle::Handle::global().app_handle().unwrap();
    let watch_center = app.state::<Mutex<ModifyWatcher>>();
    let watcher = watch_center.lock();
    if let Ok(mut watcher) = watcher {
        watcher.remove_watch(repo);
        Ok(())
    } else {
        log::error!("watcher center is not ready");
        Err(CommandError::RemoveWatcherError(repo))
    }
}


#[tauri::command]
pub fn repos() -> Result<Vec<store::Repository>, CommandError> {
    let store = handle::Handle::global().store().unwrap();
    let repos = store.get_repos();
    match repos {
        Ok(repos) => Ok(repos),
        Err(e) => Err(CommandError::FindAuthorsError(e.to_string())),
    }
}

#[tauri::command]
pub fn add_watch(repo: RepoPath) -> Result<(), CommandError> {
    watch(repo)
}

#[tauri::command]
pub fn authors(repo: RepoPath, branch: Branch) -> Result<Vec<Author>, CommandError> {
    let provider = get_provider(&repo)?;
    let authors = provider.authors(&branch);
    if let Err(_) = authors {
        return Err(CommandError::GetAuthorError(format!(
            "{} {}",
            repo, branch.name
        )));
    }
    Ok(authors.unwrap())
}

#[tauri::command]
pub fn branches(repo: RepoPath) -> Result<Vec<Branch>, CommandError> {
    let provider = get_provider(&repo)?;
    let branches = provider.branches();
    if let Err(e) = branches {
        return Err(CommandError::BranchNotFound(repo));
    }
    Ok(branches.unwrap())
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
pub fn get_db_path(db: String) -> Result<String, CommandError> {
    match db.as_str() {
        "store" => Ok(dirs::store_file().unwrap().to_str().unwrap().to_string()),
        "cache" => Ok(dirs::cache_file().unwrap().to_str().unwrap().to_string()),
        "config" => Ok(dirs::config_file().unwrap().to_str().unwrap().to_string()),
        _ => Err(CommandError::DbNotFound(db)),
    }
}

#[tauri::command]
pub fn get_driver() -> Result<Vec<Dir>, CommandError> {
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
pub fn get_folders(path: String) -> Result<Vec<Dir>, CommandError> {
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
        Err(e) => Err(CommandError::GetFoldersError(e.to_string())),
    }
}

#[tauri::command]
pub fn get_separator() -> String {
    std::path::MAIN_SEPARATOR.to_string()
}

#[tauri::command]
pub fn is_repo(repo: RepoPath) -> bool {
    is_git_repo(&repo)
}

#[tauri::command]
pub fn work_status(repo: RepoPath) -> Result<WorkStatus, CommandError> {
    let provider = get_provider(&repo)?;
    let statuses = provider.work_status();
    if let Err(e) = statuses {
        return Err(CommandError::GetWorkStatusError(e.to_string()));
    }
    Ok(statuses.unwrap())
}

#[tauri::command]
pub fn set_repo_ownership(repo: RepoPath) -> Result<bool, CommandError> {
    let provider = get_provider(&repo);
    match provider {
        Ok(_) => Ok(true),
        Err(_) => match set_owner(&repo) {
            Ok(_) => Ok(true),
            Err(err) => Err(CommandError::SetRepoOwnershipError(
                err.message().to_string(),
            )),
        },
    }
}

#[tauri::command]
pub fn branch_commits(repo: RepoPath, branch: Branch, count: i32) -> Result<Vec<Commit>, CommandError> {
    let provider = get_provider(&repo)?;
    let commits = provider.get_branch_commits(&branch, count);
    if let Err(e) = commits {
        return Err(CommandError::GetBranchCommitsError(e.to_string()));
    }
    Ok(commits.unwrap())
}

#[tauri::command]
pub fn current_branch(repo: RepoPath) -> Result<Branch, CommandError> {
    let provider = get_provider(&repo)?;
    let branch = provider.current_branch();
    if let Err(e) = branch {
        return Err(CommandError::GetCurrentBranchError(e.to_string()));
    }
    Ok(branch.unwrap())
}

#[tauri::command]
pub fn commit_content (repo: RepoPath, cid: String) -> Result<Vec<File>, CommandError> {
    let provider = get_provider(&repo)?;
    let oid = Oid::from_str(&cid);
    if let Err(e) = oid {
        return Err(CommandError::ConvertOidError(cid));
    }
    let commit_id = oid.unwrap();
    let content = provider.commit_content(commit_id);
    if let Err(e) = content {
        return Err(CommandError::GetCommitContentError(e.to_string()));
    }
    Ok(content.unwrap())
}

#[tauri::command]
pub fn file_diff(repo: RepoPath, old: String, new: String) -> Result<ContentDiff, CommandError> {
    let provider = get_provider(&repo)?;
    let old_id = Oid::from_str(&old);
    if let Err(e) = old_id {
        return Err(CommandError::ConvertOidError(old));
    }
    let old = old_id.unwrap();
    let new_id = Oid::from_str(&new);
    if let Err(e) = new_id {
        return Err(CommandError::ConvertOidError(new));
    }
    let new = new_id.unwrap();
    let diff = provider.get_file_content_diff(old, new);
    if let Err(e) = diff {
        return Err(CommandError::GetFileDiffError(e.to_string()));
    }
    Ok(diff.unwrap())
}

#[tauri::command]
pub fn blob_content(repo: RepoPath, cid: String) -> Result<Vec<u8>, CommandError> {
    let provider = get_provider(&repo)?;
    let oid = Oid::from_str(&cid);
    if let Err(e) = oid {
        return Err(CommandError::ConvertOidError(cid));
    }
    let commit_id = oid.unwrap();
    let content = provider.get_blob_content(commit_id);
    if let Err(e) = content {
        return Err(CommandError::GetFileContentError(e.to_string()));
    }
    Ok(content.unwrap())
}

#[tauri::command]
pub fn get_commit(repo: RepoPath, cid: String) -> Result<Commit, CommandError> {
    let provider = get_provider(&repo)?;
    let oid = Oid::from_str(&cid);
    if let Err(e) = oid {
        return Err(CommandError::ConvertOidError(cid));
    }
    let commit_id = oid.unwrap();
    let commit = provider.get_commit(commit_id);
    if let Err(e) = commit {
        return Err(CommandError::GetCommitError(e.to_string()));
    }
    Ok(commit.unwrap())
}

#[tauri::command]
pub fn get_branch_commit_contribution(key: String, repo: RepoPath, branch: Branch) -> Result<(), CommandError> {
    let provider = get_provider(&repo)?;
    // 由于第一次执行时间很长，所以开一个线程执行
    thread::spawn(move || {
        let contrib = provider.get_branch_commit_contribution(&branch);
        emit_branch_contribution(&key, contrib);
    });
    Ok(())
}

#[tauri::command]
pub fn get_global_author() -> Result<Author, CommandError> {
    let author = giter_utils::util::get_global_git_author();
    if let Err(e) = author {
        return Err(CommandError::GetGlobalAuthorError(e.to_string()));
    }
    Ok(author.unwrap())
}

#[tauri::command]
pub fn get_repo_author(repo: RepoPath) -> Result<Author, CommandError> {
    let provider = get_provider(&repo)?;
    let author = provider.author();
    if let Err(e) = author {
        return Err(CommandError::GetRepoAuthorError(e.to_string()));
    }
    Ok(author.unwrap())
}

#[tauri::command]
pub fn get_branch_commits_after_filter(repo: RepoPath, branch: Branch, filter: HashMap<String, Value>) -> Result<Vec<Commit>, CommandError> {
    let provider = get_provider(&repo)?;
    let commits = provider.get_branch_commits_after_filter(&branch, &filter);
    if let Err(e) = commits {
        return Err(CommandError::GetBranchCommitsError(e.to_string()));
    }
    Ok(commits.unwrap())
}