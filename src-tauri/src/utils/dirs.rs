use crate::{core::handle, types::fs::{Catalog, Dir, File}};
use anyhow::Result;
use std::path::PathBuf;
use tauri::Manager;
use giter_utils::util::is_git_repo;

use windows::Win32::Storage::FileSystem::*;
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

pub fn cache_dir() -> Result<PathBuf> {
    Ok(app_home_dir()?.join("cache"))
}

pub fn app_cache_file() -> Result<PathBuf> {
    Ok(cache_dir()?.join("cache.db"))
}

pub fn store_dir() -> Result<PathBuf> {
    Ok(app_home_dir()?.join("store"))
}

pub fn store_file() -> Result<PathBuf> {
    Ok(store_dir()?.join("store.db"))
}

pub fn app_logs_dir() -> Result<PathBuf> {
    Ok(app_home_dir()?.join("logs"))
}
