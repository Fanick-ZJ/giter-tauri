use notify::{Config, Event, RecommendedWatcher, Watcher};
use dashmap::DashMap;
use dashmap::rayon::map::Iter;  // 并行迭代器 trait
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;

use crate::error::ErrorCode;

pub struct ModifyWatcher {
    pub name: String,
    pub repos: Arc<RwLock<Vec<PathBuf>>>,
    watcher: Option<RecommendedWatcher>,
    cb_map: Arc<DashMap<usize, Box<dyn Fn(Arc<Event>) + Send + Sync>>>,
    next_id: AtomicUsize,
}

impl ModifyWatcher {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            repos: Arc::new(RwLock::new(Vec::new())),
            watcher: None,
            cb_map: Arc::new(DashMap::new()),
            next_id: AtomicUsize::new(0),
        }
    }

    pub fn init(&mut self) -> notify::Result<()> {
        if self.watcher.is_none() {
            let config = Config::default()
                .with_poll_interval(Duration::from_secs(30))
                .with_compare_contents(false);

            let cb_map = Arc::clone(&self.cb_map);
            let mut watcher = RecommendedWatcher::new(
                move |event: notify::Result<Event>| {
                    let event = match event {
                        Ok(e) => Arc::new(e),
                        Err(e) => {
                            eprintln!("Watcher error: {}", e);
                            return;
                        }
                    };

                    cb_map.iter().for_each(|entry| {
                        entry.value()(Arc::clone(&event));
                    });
                },
                config,
            )?;

            // 重新注册已有路径
            let repos = self.repos.read().clone();
            for path in &repos {
                watcher.watch(path, notify::RecursiveMode::Recursive)?;
            }

            self.watcher = Some(watcher);
        }
        Ok(())
    }

    pub fn add_watch(&mut self, p: impl Into<PathBuf>) -> Result<(), ErrorCode> {
        let path = p.into();
        let mut repos = self.repos.write();
        if !repos.contains(&path) {
            repos.push(path.clone());
            if let Some(watcher) = &mut self.watcher {
                watcher.watch(&path, notify::RecursiveMode::Recursive)
                    .map_err(|e| ErrorCode::AddWatcherFailed(e.to_string()))?;
            }
        }
        Ok(())
    }

    pub fn remove_watch(&mut self, p: impl Into<PathBuf>) -> Result<(), ErrorCode> {
        let path = p.into();
        let mut repos = self.repos.write();
        if let Some(index) = repos.iter().position(|x| x == &path) {
            repos.remove(index);
            if let Some(watcher) = &mut self.watcher {
                watcher.unwatch(&path).map_err(|e| ErrorCode::RemoveWatcherFailed(e.to_string()))?;
            }
        }
        Ok(())
    }

    pub fn add_callback<F>(&self, callback: F) -> usize
    where
        F: Fn(Arc<Event>) + 'static + Send + Sync,
    {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.cb_map.insert(id, Box::new(callback));
        id
    }

    pub fn remove_callback(&self, id: usize) {
        self.cb_map.remove(&id);
    }
}