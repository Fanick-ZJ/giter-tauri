use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::sync::Arc;
use tauri::AppHandle;

use super::cache::GitCache;


// 全局唯一实例
static HANDLE: OnceCell<Handle> = OnceCell::new();
#[derive(Debug, Default, Clone)]
pub struct Handle {
    pub app_handle: Arc<RwLock<Option<AppHandle>>>,
    pub cache: Arc<RwLock<Option<GitCache>>>,
    pub is_exiting: Arc<RwLock<bool>>
}

impl Handle {
    pub fn global() -> &'static Handle {

        HANDLE.get_or_init(|| Handle {
            app_handle: Arc::new(RwLock::new(None)),
            cache: Arc::new(RwLock::new(None)),
            is_exiting: Arc::new(RwLock::new(false)),
        })
    }

    pub fn init(&self, app_handle: &AppHandle) {
        let mut handle = self.app_handle.write();
        *handle = Some(app_handle.clone());
        drop(handle);

        let mut cache = self.cache.write();
        *cache = Some(GitCache::new());
    }


    pub fn app_handle(&self) -> Option<AppHandle> {
        self.app_handle.read().clone()
    }

    pub fn cache(&self) -> Option<GitCache> {
        self.cache.read().clone()
    }
}
