use std::{collections::HashMap, sync::Mutex, thread};
use giter_utils::types::{author::Author, branch::Branch, cache::Cache, git_data_provider::GitDataProvider};
use giter_watcher::types::modify_watcher::ModifyWatcher;
use crate::{core::{cache::GitCache, handle}, types::error::CommandError};

fn get_provider(
  repo: &str
) -> Result<GitDataProvider, CommandError> {
  let handle = handle::Handle::global();
  let provider = GitDataProvider::new(repo);
  match provider {
    Ok(mut provider) => {
      let cache = handle.cache().unwrap();
      provider.set_cache(cache);
      Ok(provider)
    }
    Err(_) => {
      return Err(CommandError::DataProviderBuildError(repo.to_string()));
    }
  }
}


#[tauri::command]
pub fn add_watch(
  path: String,
  watcher_center: tauri::State<'_, Mutex<ModifyWatcher>>,
) -> Result<(), CommandError> {

  match watcher_center.lock() {
      Ok(mut watcher) => {
        watcher.add_watch(path);
      }
      _ => {}
  }
  Ok(())
}

#[tauri::command]
pub fn authors(
  repo: String, 
  branch: Branch,
) -> Result<Vec<Author>, CommandError> {
  let provider = get_provider(&repo)?;
  let authors = provider.authors(&branch);
  if let Err(_) = authors {
    return Err(CommandError::GetAuthorError(format!("{} {}", repo, branch.name)));
  }
  Ok(authors.unwrap())
}

#[tauri::command]
pub fn branches(
  repo: String,
) -> Result<Vec<Branch>, CommandError> {;
  let provider = get_provider(&repo)?;
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