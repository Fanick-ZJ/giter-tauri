use std::{collections::HashMap, path::PathBuf, str::FromStr,};
use gix::{hashtable::hash_set::HashSet, ObjectId};
use giter_utils::types::{author::Author, branch::Branch, cache::Cache as ProviderCache};
use rusqlite::params;
use crate::{
  types::{cache::{AuthorCache, BranchAuthorCache, BranchName, RepoPath}, store},
  utils::{db::conn_db, dirs::{cache_file, repo_default_alias}}
};

#[derive(Debug, Clone)]
pub struct GitCache {
  path: PathBuf
}

impl GitCache {
  pub fn new() -> Self {
    let path = cache_file().unwrap();
    GitCache {
      path
    }
  }
}

impl GitCache {

  /// 从数据库中获取缓存
  pub fn authors_cache(&self, repo: &str) -> BranchAuthorCache {
    let conn = conn_db(self.path.clone()).unwrap();
    let mut stmt = conn.prepare("select * from branch_author where path=?1").unwrap();
    let caches = stmt.query_map(
      [repo], |row| {
        match (
          row.get::<_, String>(2),
          row.get::<_, String>(3),
          row.get::<_, String>(4))
        {
          (Ok(branch), Ok(authors), Ok(last_commit_id)) => {
            let authors: Vec<Author> = serde_json::from_str(&authors).unwrap();
            let last_commit_id = ObjectId::from_str(&last_commit_id).unwrap();
            Ok((branch, AuthorCache {
              authors: Some(authors),
              last_commit_id: Some(last_commit_id)
            }))
          },
          _ => Err(rusqlite::Error::QueryReturnedNoRows)
        }
      }
    );
    if let Err(e) = caches {
      log::error!(target: "app", "Failed to the app home directory: {}", e);
      return HashMap::new();
    }
    let cache_iter = caches.unwrap();
    let mut cache_map: HashMap<BranchName, AuthorCache> = HashMap::new();
    for item in cache_iter {
      if let Ok((branch, authors)) = item {
        cache_map.insert(branch.to_string(), authors);
      }
    }
    cache_map
  }

  /// 更新作者缓存
  fn update_author_inner(&self, repo: RepoPath, author_cache: &BranchAuthorCache) {
    let insert_sql = "Insert into branch_author (id, path, branch, authors, last_commit_id) values (null, ?1, ?2, ?3, ?4)";
    let select_sql = "select count(*) from branch_author where path=?1 and branch=?2";
    let update_sql = "update branch_author set authors=?1, last_commit_id=?2 where path=?3 and branch=?4";
    // 查询是否存在
    let conn = conn_db(self.path.clone()).unwrap();
    let mut stmt = conn.prepare(select_sql).unwrap();
    for (branch, cache) in author_cache {
      let select: Result<i32, rusqlite::Error> = stmt.query_row([repo.as_str(), branch.as_str()], |row| {
        row.get::<_, i32>(0)
      });
      if let Err(e) = select {
        log::error!(target: "app", "Failed to the app home directory: {}", e);
        continue;
      }
      // 存在则更新
      if select.unwrap() > 0 {
        let update = conn.execute(update_sql, [
          serde_json::to_string(&cache.authors).unwrap(),
          cache.last_commit_id.unwrap().to_string(),
          repo.to_string(),
          branch.to_string()
        ]);
        if let Err(e) = update {
          log::error!(target: "app", "Failed to the app home directory: {}", e)
        }
      } else {
        // 不存在则插入
        let insert = conn.execute(insert_sql, [
          repo.as_str(),
          branch.as_str(),
          serde_json::to_string(&cache.authors).unwrap().as_str(),
          cache.last_commit_id.unwrap().to_string().as_str()
        ]);
        if let Err(e) = insert {
          log::error!(target: "app", "Failed to the app home directory: {}", e)
        }
      }
    }
  }

  pub fn update_author(&self, repo: RepoPath, author_cache: &BranchAuthorCache) {
    self.update_author_inner(repo, author_cache);
  }

  /// 获取分支作者缓存
  pub fn branch_authors_inner(&self, repo: RepoPath, branch: BranchName) -> Option<(Vec<Author>, ObjectId)> {
    let conn = conn_db(self.path.clone()).unwrap();
    let mut stmt = conn.prepare("select * from branch_author where path=?1 and branch=?2").unwrap();
    let caches = stmt.query_row(
      [repo, branch], |row| {
        match (row.get::<_, String>(3), row.get::<_, String>(4)) {
          (Ok(authors), Ok(last_commit_id)) => {
            let authors: Vec<Author> = serde_json::from_str(&authors).unwrap();
            let last_commit_id = ObjectId::from_str(&last_commit_id).unwrap();
            Ok((authors,last_commit_id))
          },
          _ => Err(rusqlite::Error::QueryReturnedNoRows)
        }
      }
    );
    if let Err(e) = caches {
      log::error!(target: "app", "Failed to the app home directory: {}", e);
      return None;
    }
    Some(caches.unwrap())

  }

  /// 清除所有作者缓存
  pub fn clear_author_cache(&self) {
    let conn = conn_db(self.path.clone()).unwrap();
    let clear_sql = "delete from branch_author where";
    let clear = conn.execute(clear_sql, []);
    match clear {
      Ok(count) => { log::info!("clear author cache success: {}", count)}
      Err(e) => {
        log::error!("clear author cache error: {:?}", e);
      }
    }
  }

  /// 清除指定仓库的作者缓存
  pub fn clear_repo_author_cache(&self, repo: RepoPath) {
    let conn = conn_db(self.path.clone()).unwrap();
    let clear_sql = "delete from branch_author where path=?1";
    let clear = conn.execute(clear_sql, [repo]);
    match clear {
      Ok(count) => { log::info!("clear author cache success: {}", count);}
      Err(e) => {
        log::error!("clear author cache error: {:?}", e);
      }
    }
  }

  /// 清除所有缓存
  pub fn clear_inner(&self) {
    self.clear_author_cache();
  }
}

impl ProviderCache for GitCache {

  fn branch_authors(&self, repo: &str, branch: &Branch) -> Option<(Vec<Author>, ObjectId)> {
    let author_cache = self.branch_authors_inner(repo.to_string(), branch.name.to_string());
    author_cache
  }
  
  fn set_authors(&mut self, repo: &str, authors: &Vec<Author>, branch: &Branch, last_commit_id: &ObjectId) {
    let author_cache = AuthorCache {
      authors: Some(authors.clone()),
      last_commit_id: Some(last_commit_id.clone())
    };
    let map = HashMap::from([(branch.name.clone(), author_cache)]);
    self.update_author(repo.to_string(), &map);
  }
  
  fn clear(&mut self, repo: &str) {
    self.clear_repo_author_cache(repo.to_string());
  }
    
  fn clear_all(&mut self) {
      self.clear_inner();
  }
  
  fn authors(&self, repo: &str) -> Option<Vec<Author>> {
    let sql = "select * from branch_author where path=?1";
    let mut author_set: HashSet<Author> = HashSet::new();
    let conn = conn_db(self.path.clone()).unwrap();
    let mut stmt = conn.prepare(sql).unwrap();
    let caches = stmt.query_map(
      [repo], |row| {
        match row.get::<_, String>(3) {
          Ok(authors) => {
            let authors: Vec<Author> = serde_json::from_str(&authors).unwrap();
            Ok(authors)
          },
          _ => Err(rusqlite::Error::QueryReturnedNoRows)
        }
      }
    );
    if let Err(e) = caches {
      println!("get cache error: {:?}", e);
      return None;
    }
    let cache_iter = caches.unwrap();
    for item in cache_iter {
      if let Ok(authors) = item {
        for author in authors {
          author_set.insert(author);
        }
      }
    }
    Some(author_set.into_iter().collect())
  }
}
