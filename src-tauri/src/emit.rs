use crate::core::handle;
use crate::SingleRepoSubmit;
use giter_utils::types::{contribution::CommitStatistic, git_data_provider::GitDataProvider, status::WorkStatus};
use giter_watcher::modify_watcher::ModifyWatcher;
use notify::Event;
use serde::Serialize;
use std::{collections::hash_set::HashSet, sync::Arc};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};

#[derive(Serialize, Debug, Clone)]
struct Status {
    path: String,
    status: WorkStatus,
}

fn get_event_repo_paths(app: &AppHandle, event: Arc<Event>) -> HashSet<PathBuf> {
    let watcher = app.state::<Mutex<ModifyWatcher>>();            
    let repos_lock = watcher.lock().unwrap_or_else(|poison| poison.into_inner());
    let repos = repos_lock.repos.read().clone();
    let paths = &event.paths;
    let mut repo_set: HashSet<PathBuf> = HashSet::new();
    // 过滤出需要更新的仓库
    for repo in repos {
        for path in paths.iter() {
            if path.starts_with(&repo) {
                repo_set.insert(repo.clone());
            }
        }
    }
    repo_set
}

/// 仓库监控到文件修改后执行的回调函数
///
pub fn satatus_change_emit(event: Arc<Event>) {
    let app = handle::Handle::global().app_handle();
    if let None = app {
        log::error!("satatus_change_emit: app is none");
        return;
    }
    let app = app.unwrap();
    let repo_set = get_event_repo_paths(&app, event);
    // 遍历仓库，更新状态
    for path in repo_set.iter() {
        let provider = GitDataProvider::new(path);
        if provider.is_err() { 
            log::error!("satatus_change_emit: provider build error: {:?}", path);
            continue; 
        }
        let status = provider.unwrap().work_status();
        if let Ok(status) = status {
            app.emit(
                "giter://status_changed",
                Status {
                    path: path.display().to_string(),
                    status,
                },
            )
            .expect("TODO: panic message");
        } else {
            log::error!("status: {:?}", status);
        }
    }
}

pub fn changed_emit(event: Arc<Event>) {
    let app = handle::Handle::global().app_handle();
    if let None = app {
        log::error!("changed_emit: app is none");
        return;
    }
    let app = app.unwrap();
    let repo_set = get_event_repo_paths(&app, event);
    // 遍历仓库，发送文件修改事件
    for path in repo_set.iter() {
        let provider = GitDataProvider::new(path);
        if provider.is_err() {
            log::error!("changed_emit: provider build error: {:?}", path);
            continue;
        }
        app.emit("giter://changed_emit", path.display().to_string())
            .expect("TODO: panic message");
    }
}

pub fn repo_single_emit(event: Arc<Event>) {
    let app = handle::Handle::global().app_handle();
    if let None = app {
        log::error!("changed_emit: app is none");
        return;
    }
    let app = app.unwrap();
    let map_lock = app.state::<SingleRepoSubmit>();
    let map = map_lock.inner().0.write();
    let repo_set = get_event_repo_paths(&app, event);
    for (repo_path, _) in map.iter() {
        let  path = PathBuf::from(repo_path);
        if repo_set.contains(&path) {
            app.emit(&format!("giter://repo_single_emit:{}", repo_path.replace("\\", "/")), ())
               .expect("TODO: panic message");
        }
    }

}

pub fn emit_branch_contribution(key: &str, value: anyhow::Result<Vec<CommitStatistic>>) {
    let app = handle::Handle::global().app_handle().unwrap();
    if let Err(e) = value {
        let _ = app.emit(&format!("giter://branch_contribution/{}", key), e.to_string());
    }
    else {
        let _ = app.emit(&format!("giter://branch_contribution/{}", key), value.unwrap());
    }
}