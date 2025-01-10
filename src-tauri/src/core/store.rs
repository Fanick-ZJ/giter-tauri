use std::{collections::HashMap, path::Path, sync::Arc};

use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};

use super::handle;
use crate::utils::consts::GIT_CACHE;

/// 获取Git缓存  tauri的store插件
/// 
pub fn git_cache_store(repo: &str) -> Arc<Store<Wry>> {
  let app = handle::Handle::global().app_handle().unwrap();
  let path = Path::new(repo);

  let store = app.store(format!("{}.json", path.file_name().unwrap().to_string_lossy()))
    .expect("store init error");
  store
}