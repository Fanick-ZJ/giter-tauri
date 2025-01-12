use std::collections::HashMap;

use giter_utils::types::author::Author;
use gix::ObjectId;
use serde::{Deserialize, Serialize};

type RepoAlias = String;
type RepoPath = String;
type BranchName = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BranchAuthorCache {
  pub authors: Option<Vec<Author>>,
  pub last_commit_id: Option<ObjectId>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorCache {
  // 各个分支的作者列表
  pub branch_authors: HashMap<RepoPath, HashMap<BranchName, BranchAuthorCache>>
}

impl AuthorCache {
  /// 根据仓库和分支获取作者列表
  pub fn get_authors(&self, repo: &str, branch: &str) -> Option<&BranchAuthorCache> {
    let branch_authors = self.branch_authors.get(repo)?;
    let authors = branch_authors.get(branch);
    if let None = authors {
      return None;
    }
    Some(authors.unwrap())
  }

  /// 根据仓库和分支设置作者列表
  pub fn set_authors(&mut self, repo: &str, branch: &str, authors: &Vec<Author>, last_commit_id: &ObjectId) {
    let branch_authors = self.branch_authors.entry(repo.to_string()).or_insert(HashMap::new());
    let old_value = branch_authors.entry(branch.to_string()).or_insert(BranchAuthorCache {
      authors: None,
      last_commit_id: None
    });
    old_value.authors = Some(authors.clone());
    old_value.last_commit_id = Some(last_commit_id.clone());
  }
}

impl AuthorCache {
  pub fn new() -> Self {
    AuthorCache {
      branch_authors: HashMap::new()
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
  pub alias: RepoAlias,
  pub path: String,
  pub has_watch: bool,
  pub order: Option<i32>,
  pub top: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppCache {
  pub repos: Vec<Repository>,
  pub authors: AuthorCache
}

impl AppCache {
  pub fn new() -> Self {
    AppCache {
      repos: Vec::new(),
      authors: AuthorCache::new()
    }
  }
}
