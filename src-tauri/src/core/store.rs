use std::{path::Path, sync::Arc};

use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};

use super::handle;

/// 获取Git缓存  tauri的store插件，保存各个仓库的单独数据
/// 
pub fn git_cache_store(repo: &str) -> Arc<Store<Wry>> {
  let app = handle::Handle::global().app_handle().unwrap();
  let path = Path::new(repo);

  let store = app.store(format!("{}.json", path.file_name().unwrap().to_string_lossy()))
    .expect("store init error");
  store
}

/// 获取仓库缓存，所有仓库的缓存都在一个文件中，例如记录的仓库列表
pub fn repository_cache_store() -> Arc<Store<Wry>> {
  let app = handle::Handle::global().app_handle().unwrap();
  let store = app.store("reository.json", )
   .expect("store init error");
  store
}