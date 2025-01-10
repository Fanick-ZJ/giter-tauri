use std::{rc::Rc, sync::Arc};
use anyhow::{Ok, Result};
use giter_utils::types::{author::{self, Author}, cache::Cache};
use gix::ObjectId;
use serde::{Deserialize, Serialize};
use tauri::Wry;
use tauri_plugin_store::Store;

use super::store::git_cache_store;

#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoryCache {
    authors_cache: Option<AuthorCache>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorCache {
    authors: Vec<Author>,
    last_commit_id: gix::ObjectId
}


pub struct GitCache {
    store: Arc<Store<Wry>>
}
impl GitCache {
    pub fn new(repo: &str) -> Self {
        Self {
            store: git_cache_store(repo)
        }
    }
}

impl Cache for GitCache {
    fn authors(&self, repo: &str) -> Result<(Vec<Author>, ObjectId)> {
        println!("getting authors from cache: {}", repo);
        let cache = self.store.get(repo);
        if let None = cache {
            return Err(anyhow::anyhow!("repo cache not found"));
        }
        let cache = cache.unwrap();
        let cache: RepositoryCache = serde_json::from_str(&cache.to_string()).unwrap();
        if let None = cache.authors_cache {
            return Err(anyhow::anyhow!("repo cache not found"));
        }
        else {
            let author_cache = cache.authors_cache.unwrap();
            Ok((author_cache.authors, author_cache.last_commit_id))
        }
        
    }
    
    fn set_authors(&self, repo: &str, contributors: &Vec<Author>, last_commit_id: &ObjectId) {
        let mut cache = RepositoryCache {
            authors_cache: None
        };
        cache.authors_cache = Some(AuthorCache {
            authors: contributors.to_vec(),
            last_commit_id: *last_commit_id
        });
        self.store.set(repo, serde_json::json!(&cache));
    }
}