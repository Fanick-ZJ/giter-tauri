use std::{collections::HashSet, io::Write, path::PathBuf, thread};
use gix::ObjectId;

use giter_utils::types::{author::Author, branch::Branch, cache::Cache as ProviderCache};
use windows::Win32::Foundation::ERROR_PRINTER_DRIVER_DOWNLOAD_NEEDED;
use crate::{
  types::cache::{AppCache, Repository}, 
  utils::{dirs::cache_dir, fs::read_json_file}
};

#[derive(Debug, Clone)]
pub struct GitCache {
  path: PathBuf,
  cache: AppCache,
}

impl GitCache {
  pub fn new() -> Self {
    let path = cache_dir().unwrap().join("cache.json");
    let cache= read_json_file::<AppCache>(&path).unwrap_or_else(|_| {
      AppCache::new()
    });
    GitCache {
      path,
      cache
    }
  }
}

impl GitCache {
  pub fn update_cache(&self) {
    let cache = serde_json::json!(self.cache);
    if !cache_dir().unwrap().exists() {
      std::fs::create_dir_all(&cache_dir().unwrap()).unwrap();
    }
    let mut file = std::fs::File::create(&self.path).unwrap();
    file.write_all(cache.to_string().as_bytes()).unwrap_or_else(|e| {
      println!("write cache failed: {:?}", e);
    });
  }
}

impl ProviderCache for GitCache {

  fn authors(&self, _repo: &str) -> Option<Vec<Author>> {
    let author_cache = &self.cache.authors;
    let mut author_set: HashSet<Author> = HashSet::new();
    for (_repo, cache) in &author_cache.branch_authors {
      for (_, branch_cache) in cache {
        if let Some(authors) = &branch_cache.authors {
          author_set.extend(authors.clone());
        }
      }
    }
    Some(author_set.into_iter().collect())
  }

  fn branch_authors(&self, repo: &str, branch: &Branch) -> Option<(Vec<Author>, ObjectId)> {
    let author_cache = &self.cache.authors;
    let branch_author = author_cache.get_authors(repo, branch.name.as_str());
    println!("branch_author: {:?}", branch_author);
    if let None = branch_author {
      return None;
    }
    let branch_author = branch_author.unwrap();
    let authors = branch_author.authors.clone().unwrap();
    let last_commit_id = branch_author.last_commit_id.clone().unwrap();
    Some((authors, last_commit_id))
  }
  
  fn set_authors(&mut self, repo: &str, authors: &Vec<Author>, branch: &Branch, last_commit_id: &ObjectId) {
    self.cache.authors.set_authors(repo, &branch.name, authors, last_commit_id);
    self.update_cache();
  }
  
  fn clear(&mut self, repo: &str) {
    self.cache.authors.branch_authors.remove(repo);
    self.update_cache();
    }
    
  fn clear_all(&mut self) {
      println!("clear cache: {:p}", &self);
      self.cache = AppCache::new();
      self.update_cache();
  }
}

impl GitCache {

  fn repos(&self) -> &Vec<Repository> {
    &self.cache.repos
  }

  fn set_repos(&mut self, repos: &Vec<Repository>) {
    self.cache.repos = repos.clone();
    self.update_cache();
  }

  fn add_repo(&mut self, repo: &Repository) {
    for r in &self.cache.repos {
      if r.path == repo.path {
        return;
      }
    }
    self.cache.repos.push(repo.clone());
  }

}