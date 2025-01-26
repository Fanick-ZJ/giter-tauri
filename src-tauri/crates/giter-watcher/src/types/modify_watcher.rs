use notify::{Config, Event, RecommendedWatcher, Watcher};
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync:: Arc;
use std::time::Duration;

pub struct ModifyWatcher {
    pub name: String,
    pub repos: Arc<RwLock<Vec<PathBuf>>>,
    watcher: Option<RecommendedWatcher>,
    cb: Option<Arc<dyn FnMut(Event) + Send + Sync>>,
}

impl ModifyWatcher {
    pub fn new() -> Self {
        let repos: Arc<RwLock<Vec<PathBuf>>> = Arc::new(RwLock::new(Vec::new()));

        Self {
            name: "".to_string(),
            repos,
            watcher: None,
            cb: None,
        }
    }
    ///将某个地址加入监听
    ///
    fn watch<'a>(&mut self, p: impl Into<&'a PathBuf>) {
        let path = p.into();
        match self.watcher {
            Some(ref mut watcher) => {
                watcher
                    .watch(path, notify::RecursiveMode::Recursive)
                    .unwrap();
            }
            None => {}
        }
    }

    pub fn init(&mut self, cb: fn(notify::event::Event)) {
        if self.watcher.is_none() {
            // 默认配置
            let config = Config::default()
                .with_poll_interval(Duration::from_secs(3))
                .with_compare_contents(false);
            // 新建文件监听器
            let cb = Arc::new(cb);
            let cb_clone = Arc::clone(&cb);
            let watcher = RecommendedWatcher::new(
                move |event: notify::Result<Event>| {
                    cb_clone(event.unwrap());
                },
                config,
            )
            .unwrap();
            self.watcher = Some(watcher);
            self.cb = Some(cb);
            // 添加监听路径
            let mut repos = self.repos.write();
            repos.clear();
        }
    }

    pub fn add_watch(&mut self, p: impl Into<PathBuf>) {
        let path = p.into();
        if self.repos.read().contains(&path) {
            return;
        }
        self.repos.write().push(path.clone());
        self.watch(&path);
    }
}
