use crate::core::handle;
use crate::emit::repos_modified_emit_cb;
use crate::utils::init;
use giter_utils::types::git_data_provider::GitDataProvider;
use giter_watcher::types::modify_watcher::ModifyWatcher;
use std::collections::HashMap;
use std::sync::Mutex;
use anyhow::Result;
use tauri::{App, Manager};

pub async fn resolve_setup(app: &mut App) -> Result<()> {
    handle::Handle::global().init(app.app_handle());
    // 初始化日志
    init::init_log()?;
    let mut watcher_center = ModifyWatcher::new();
    watcher_center.init(repos_modified_emit_cb());
    // 修改监控器
    app.manage(Mutex::new(watcher_center));
    // 各个仓库监控器
    app.manage(Mutex::new(HashMap::<String, GitDataProvider>::new()));

    Ok(())
}
