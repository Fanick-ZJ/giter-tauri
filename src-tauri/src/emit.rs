use crate::core::handle;
use giter_utils::types::{
    git_data_provider::GitDataProvider,
    status::{WorkStatus},
};
use giter_watcher::types::modify_watcher::ModifyWatcher;
use notify::Event;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{collections::hash_set::HashSet};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Emitter, Manager};

/// 仓库监控到文件修改后执行的回调函数
///
pub fn repos_modified_emit_cb() -> impl Fn(Event) {
    #[derive(Serialize, Debug, Deserialize, Clone)]
    struct Status {
        path: String,
        status: WorkStatus,
    }
    move |event| {
        let app = handle::Handle::global().app_handle().unwrap();
        let watcher = app.state::<Mutex<ModifyWatcher>>();
        let providers = app.state::<Mutex<HashMap<String, GitDataProvider>>>();
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
            let status = providers
                .lock()
                .unwrap()
                .get(path.to_str().unwrap())
                .unwrap()
                .file_status()
                .unwrap();
            println!("{:?}", path);
            app.emit(
                "emit_test",
                Status {
                    path: path.to_str().unwrap().to_string(),
                    status,
                },
            );
        }

        // app.emit("emit_test", event.paths);
    }
}
