use std::sync::Mutex;
use std::collections::HashMap;
use tauri::{App, Emitter, Manager};
use giter_utils::types::git_data_provider::GitDataProvider;
use giter_watcher::types::modify_watcher::ModifyWatcher;
use crate::core::handle;
use crate::emit::repos_modified_emit_cb;
use crate::utils::init;

pub async fn resolve_setup(app: &mut App) {
    handle::Handle::global().init(app.app_handle());
    init::init_log();
    let mut watcher_center = ModifyWatcher::new();
    watcher_center.init(repos_modified_emit_cb());
    // 修改监控器
    app.manage(Mutex::new(watcher_center));
    // 各个仓库监控器
    app.manage(Mutex::new(HashMap::<String, GitDataProvider>::new()));
}