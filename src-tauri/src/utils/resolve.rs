use crate::{core::handle, emit::changed_emit};
use crate::emit::satatus_change_emit;
use crate::utils::init;
use anyhow::Result;
use giter_watcher::modify_watcher::ModifyWatcher;
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
    watcher_center.add_callback(satatus_change_emit);
    watcher_center.add_callback(changed_emit);
    let _ = watcher_center.init();
    // 修改监控器
    app.manage(Mutex::new(watcher_center));

    Ok(())
}
