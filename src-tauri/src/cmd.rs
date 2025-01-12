use std::{collections::HashMap, sync::Mutex, thread};
use giter_utils::types::{author::Author, branch::Branch, cache::Cache, git_data_provider::GitDataProvider};
use giter_watcher::types::modify_watcher::ModifyWatcher;
use crate::{core::{cache::GitCache, handle}, types::error::CommandError};

fn get_provider<'a>(
  repo: &str,
  data_provider: &'a HashMap<String, GitDataProvider>
) -> Result<&'a GitDataProvider, CommandError> {
  // let provider = data_provider.lock().unwrap();
  data_provider.get(repo).ok_or(CommandError::DataProviderNotExist(repo.to_string()))
}


#[tauri::command]
pub fn add_watch(
  path: String,
  watcher_center: tauri::State<'_, Mutex<ModifyWatcher>>,
  data_providers: tauri::State<'_, Mutex<HashMap<String, GitDataProvider>>>
) -> Result<(), CommandError> {

  match (watcher_center.lock(), data_providers.lock()) {
      (Ok(mut watcher), Ok(mut providers)) => {
          // 判断是否已经加载过了
          if let None = providers.get(&path) {
              let provider = GitDataProvider::new(&path);
              if let Ok(mut provider) = provider {
                  let cache = handle::Handle::global().cache().unwrap();
                  println!("clone cache: {:p}", &cache);
                  provider.set_cache(cache);
                  println!("add watch: {:p}", provider);
                  providers.insert(path.clone(), provider);
                  watcher.add_watch(path);
              } else {
                  // 非法路径
                  return Err(CommandError::InvalidRepository(path));
              }
          } else {
              return Err(CommandError::RepositoryHasWatched(path));
          }
      }
      _ => {}
  }
  Ok(())
}

#[tauri::command]
pub fn authors(
  repo: String, 
  branch: Branch,
  data_provider: tauri::State<'_, Mutex<HashMap<String, GitDataProvider>>>
) -> Result<Vec<Author>, CommandError> {
  let provider = data_provider.lock().unwrap();
  let provider = get_provider(&repo, &provider).unwrap();
  let authors = provider.authors(&branch);
  if let Err(_) = authors {
    return Err(CommandError::GetAuthorError(format!("{} {}", repo, branch.name)));
  }
  Ok(authors.unwrap())
}

#[tauri::command]
pub fn branches(
  repo: String,
  data_provider: tauri::State<'_, Mutex<HashMap<String, GitDataProvider>>>
) -> Result<Vec<Branch>, CommandError> {
  let provider = data_provider.lock().unwrap();
  let provider = get_provider(&repo, &provider).unwrap();

  let branches = provider.branches();
  if let Err(e) = branches {
    return Err(CommandError::BranchNotFound(repo));
  }
  Ok(branches.unwrap())
}

#[tauri::command]
pub fn clear_cache(repo: String) {
  let mut cache = handle::Handle::global().cache().unwrap();
  cache.clear(&repo);
}

#[tauri::command]
pub fn clear_all_cache() {
  let mut cache = handle::Handle::global().cache().unwrap();
  cache.clear_all();
}