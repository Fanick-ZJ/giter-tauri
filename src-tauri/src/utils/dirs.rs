use crate::{core::handle, types::cache::RepoPath};
use anyhow::Result;
use std::path::PathBuf;
use tauri::Manager;

static DOT_GIT: &str = ".git";

pub static APP_ID: &str = "giter";

pub fn app_home_dir() -> Result<PathBuf> {
    let app_handle = handle::Handle::global().app_handle().unwrap();
    match app_handle.path().data_dir() {
        Ok(dir) => Ok(dir.join(APP_ID)),
        Err(e) => {
            log::error!(target: "app", "Failed to the app home directory: {}", e);
            Err(anyhow::anyhow!("Failed to the app home directory"))
        }
    }
}

pub fn database_dir() -> Result<PathBuf> {
    Ok(app_home_dir()?.join("database"))
}

pub fn cache_file() -> Result<PathBuf> {
    Ok(database_dir()?.join("cache.db"))
}

pub fn store_file() -> Result<PathBuf> {
    Ok(database_dir()?.join("store.db"))
}

pub fn config_file() -> Result<PathBuf> {
    Ok(database_dir()?.join("config.db"))
}

pub fn app_logs_dir() -> Result<PathBuf> {
    Ok(app_home_dir()?.join("logs"))
}

pub fn is_dot_git_dir(dir: &RepoPath) -> bool {
    let dir = PathBuf::from(dir);
    dir.is_absolute() && dir.ends_with(DOT_GIT)
}

pub fn repo_default_alias(repo: &RepoPath) -> String {
    let _repo = PathBuf::from(repo);
    if is_dot_git_dir(repo) {
        _repo
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else {
        _repo.file_name().unwrap().to_str().unwrap().to_string()
    }
}
