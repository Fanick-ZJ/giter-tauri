use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::sync::Arc;
use tauri::AppHandle;

#[derive(Debug, Default, Clone)]
pub struct Handle {
    pub app_handle: Arc<RwLock<Option<AppHandle>>>,
    pub is_exiting: Arc<RwLock<bool>>
}

impl Handle {
    pub fn global() -> &'static Handle {
        // 全局唯一实例
        static HANDLE: OnceCell<Handle> = OnceCell::new();

        HANDLE.get_or_init(|| Handle {
            app_handle: Arc::new(RwLock::new(None)),
            is_exiting: Arc::new(RwLock::new(false)),
        })
    }

    pub fn init(&self, app_handle: &AppHandle) {
        let mut handle = self.app_handle.write();
        *handle = Some(app_handle.clone());
    }


    pub fn app_handle(&self) -> Option<AppHandle> {
        self.app_handle.read().clone()
    }
}
