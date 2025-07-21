use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::sync::Arc;
use tauri::AppHandle;

use super::store::GitStore;

// 全局唯一实例
static HANDLE: OnceCell<Handle> = OnceCell::new();
#[derive(Debug, Default, Clone)]
pub struct Handle {
    pub app_handle: Arc<RwLock<Option<AppHandle>>>,
    pub store: Arc<RwLock<Option<GitStore>>>,
    pub is_exiting: Arc<RwLock<bool>>,
}

impl Handle {
    pub fn global() -> &'static Handle {
        HANDLE.get_or_init(|| Handle {
            app_handle: Arc::new(RwLock::new(None)),
            store: Arc::new(RwLock::new(None)),
            is_exiting: Arc::new(RwLock::new(false)),
        })
    }

    pub fn init(&self, app_handle: &AppHandle) {
        let mut handle = self.app_handle.write();
        *handle = Some(app_handle.clone());
        drop(handle);

        let mut store = self.store.write();
        *store = Some(GitStore::new());
    }

    pub fn app_handle(&self) -> Option<AppHandle> {
        self.app_handle.read().clone()
    }

    pub fn store(&self) -> Option<GitStore> {
        self.store.read().clone()
    }
}
