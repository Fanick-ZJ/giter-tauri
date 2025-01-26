use crate::core::handle;
use giter_utils::types::{
    git_data_provider::GitDataProvider,
    status::WorkStatus,
};
use giter_watcher::types::modify_watcher::ModifyWatcher;
use notify::Event;
use serde::{Deserialize, Serialize};
use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Emitter, Manager};

/// 仓库监控到文件修改后执行的回调函数
///
pub fn repos_modified_emit_cb() -> fn(Event) {
    #[derive(Serialize, Debug, Deserialize, Clone)]
    struct Status {
        path: String,
        status: WorkStatus,
    }
    move |event: Event| {
        let app = handle::Handle::global().app_handle().unwrap();
        let watcher = app.state::<Mutex<ModifyWatcher>>();
        let paths = event.paths;
        let mut repo_set: HashSet<PathBuf> = HashSet::new();
        for repo in watcher.lock().unwrap().repos.read().iter() {
            for path in paths.iter() {
                if path.starts_with(repo) {
                    repo_set.insert(repo.clone());
                }
            }
        }

        for path in repo_set.iter() {
            let path = path.to_str().unwrap();
            let provider = GitDataProvider::new(path);
            let status = provider.unwrap().work_status();
            if let Ok(status) = status {
                app.emit(
                    "giter://status_changed",
                    Status {
                        path: path.to_string(),
                        status,
                    },
                ).expect("TODO: panic message");
            } else {
                log::error!("status: {:?}", status);
            }
        }

        // app.emit("emit_test", event.paths);
    }
}
