use std::{collections::HashMap, sync::Arc};
use std::collections::HashSet;
use giter_utils::types::{author::Author, branch::Branch, cache::Cache};
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
pub struct BranchAuthorCache {
    // branch: Branch,
    authors: Option<Vec<Author>>,
    last_commit_id: Option<ObjectId>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorCache {
    // 各个分支的作者列表
    branch_authors: HashMap<String, BranchAuthorCache>
}

impl AuthorCache {
    pub fn new() -> Self {
        AuthorCache {
            branch_authors: HashMap::new()
        }
    }
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

impl GitCache {
    fn get_author_cache_meta(&self) -> AuthorCache {
        let cache = self.store.get("authors");
        if let None = cache {
            return AuthorCache::new()
        }
        let author_cache = serde_json::from_str::<AuthorCache>(&cache.unwrap().to_string());
        if let Err(_) = author_cache {
            return AuthorCache::new()
        }
        author_cache.unwrap()
    }
}

impl Cache for GitCache {

    fn authors(&self, repo: &str) -> Option<Vec<Author>> {
        let author_cache = self.get_author_cache_meta();
        let mut author_set = HashSet::new();
        for (_branch, cache) in author_cache.branch_authors {
            if let Some(author) = cache.authors {
                author_set.extend(author);
            }
        }
        Some(author_set.into_iter().collect())
    }

    fn branch_authors(&self, repo: &str, branch: &Branch) -> Option<(Vec<Author>, ObjectId)> {
        let author_cache = self.get_author_cache_meta();
        let branch_author = author_cache.branch_authors.get(branch.name.as_str());
        if let None = branch_author {
            return None
        }
        let branch_author = branch_author.unwrap();
        if let None = branch_author.authors {
            return None
        }
        let authors = branch_author.authors.clone().unwrap();
        let last_commit_id = branch_author.last_commit_id.unwrap();
        Some((authors, last_commit_id))
    }
    
    fn set_authors(&self, repo: &str, authors: &Vec<Author>, branch: &Branch, last_commit_id: &ObjectId) {
        let mut cache = self.get_author_cache_meta();
        let branch_info =cache.branch_authors.entry(branch.name.to_string()).or_insert(BranchAuthorCache {
            // branch: branch.clone(),
            authors: None,
            last_commit_id: None
        });
        branch_info.authors = Some(authors.clone());
        branch_info.last_commit_id = Some(last_commit_id.clone());
        self.store.set("authors", serde_json::json!(&cache));
    }
}