use crate::core::handle;
use crate::emit::repos_modified_emit_cb;
use crate::utils::init;
use anyhow::Result;
use giter_watcher::types::modify_watcher::ModifyWatcher;
use std::sync::Mutex;
use tauri::{App, Manager};

pub async fn resolve_setup(app: &mut App) -> Result<()> {
    handle::Handle::global().init(app.app_handle());
    // 初始化日志
    init::init_log()?;
    println!("init log success");
    init::init_cache()?;
    println!("init cache success");
    init::init_store()?;
    println!("init store success");
    init::init_config()?;
    println!("init conifg success");
    let mut watcher_center = ModifyWatcher::new();
    watcher_center.init(repos_modified_emit_cb());
    // 修改监控器
    app.manage(Mutex::new(watcher_center));

    Ok(())
}
