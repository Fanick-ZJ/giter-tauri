use std::sync::Mutex;
use giter_utils::types::{author::Author, branch::Branch, cache::Cache, git_data_provider::GitDataProvider};
use giter_watcher::types::modify_watcher::ModifyWatcher;
use tauri::Manager;
use crate::{core::handle, types::{cache::RepoPath, error::CommandError, fs::Dir, store}, utils::{dirs, fs::{get_first_level_dirs, get_logical_driver}}};

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

fn watch(repo: RepoPath) -> Result<(), CommandError> {
  let app = handle::Handle::global().app_handle().unwrap();
  let watch_center = app.state::<Mutex<ModifyWatcher>>();
  let watcher = watch_center.lock();
  if let Ok(mut watcher) = watcher {
    watcher.add_watch(repo);
    Ok(())
  } else {
    log::error!("watcher center is not ready");
    Err(CommandError::AddWatcherError(repo))
  }
}

#[tauri::command]
pub fn add_repo(
  path: String,
  alias: Option<String>,
  has_watch: Option<bool>,
  order: Option<i32>,
  top: Option<bool>
) -> Result<store::Repository, CommandError> {
  let handle = handle::Handle::global();
  let store = handle.store().unwrap();
  let repo = store.add_repo(path, alias, has_watch, order, top);
  match repo {
    Ok(repo) => {
      if repo.has_watch {
        watch(repo.path.clone())?;
      }
      Ok(repo)
    }
    Err(e) => {
      Err(CommandError::AddRepositoryStoreError(e.to_string()))
    }
  }
}

#[tauri::command]
pub fn repos() -> Result<Vec<store::Repository>, CommandError> {
  let store = handle::Handle::global().store().unwrap();
  let repos = store.get_repos();
  match repos {
    Ok(repos) => Ok(repos),
    Err(e) => Err(CommandError::FindAuthorsError(e.to_string()))
  }
}

#[tauri::command]
pub fn add_watch(
  path: String
) -> Result<(), CommandError> {
  watch(path)
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

#[tauri::command]
pub fn get_db_path(db: String) -> Result<String, CommandError> {
  match db.as_str() {
    "store" => Ok(dirs::store_file().unwrap().to_str().unwrap().to_string()),
    "cache" => Ok(dirs::cache_file().unwrap().to_str().unwrap().to_string()),
    "config" => Ok(dirs::config_file().unwrap().to_str().unwrap().to_string()),
    _ => Err(CommandError::DbNotFound(db))
  }
}

#[tauri::command]
pub fn get_drive() -> Result<Vec<Dir>, CommandError> {
  let driver = get_logical_driver();
  let mut folders = vec![];
  for item in driver {
    // 舍去最后两个//
    let name = item.chars().take(item.len() - 1).collect();
    folders.push(Dir {
      name,
      path: item,
      is_repo: false,
    });
  }
  Ok(folders)
}

#[tauri::command]
pub fn get_folders(path: String) -> Result<Vec<Dir>, CommandError> {
  let catalog = get_first_level_dirs(&path);
  match catalog {
    Ok(catalog) => {
      let mut folders = vec![];
      for item in catalog.dirs {
        folders.push(Dir {
          name: item.name,
          path: item.path,
          is_repo: item.is_repo,
        });
      }
      Ok(folders)
    },
    Err(e) => Err(CommandError::GetFoldersError(e.to_string()))
  }
}

#[tauri::command]
pub fn get_separator() -> String {
  std::path::MAIN_SEPARATOR.to_string()
}