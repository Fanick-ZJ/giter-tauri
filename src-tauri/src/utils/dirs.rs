use std::path::PathBuf;
use anyhow::Result;
use tauri::Manager;
use crate::core::handle;

pub static APP_ID: &str = "giter";
pub static BACKUP_DIR: &str = "giter-backup";

pub fn app_home_dir() -> Result<PathBuf> {
    use tauri::utils::platform::current_exe;

    let app_handle = handle::Handle::global().app_handle().unwrap();
    match app_handle.path().data_dir() {
        Ok(dir) => Ok(dir.join(APP_ID)),
        Err(e) => {
            log::error!(target: "app", "Failed to the app home directory: {}", e);
            Err(anyhow::anyhow!("Failed to the app home directory"))
        },
    }
}

pub fn app_logs_dir() -> Result<PathBuf> {
    Ok(app_home_dir()?.join("logs"))
}