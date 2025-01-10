use std::{collections::HashMap, sync::Mutex};
use giter_utils::types::{author::Author, branch::Branch, git_data_provider::GitDataProvider};
use crate::{types::error::CommandError};

#[tauri::command]
pub fn authors(
  repo: String, 
  branch: Branch,
  data_provider: tauri::State<'_, Mutex<HashMap<String, GitDataProvider>>>
) -> Result<Vec<Author>, CommandError> {
  let mut provider = data_provider.lock().unwrap();
  let provider = provider.get_mut(&repo);
  if provider.is_none() {
    return Err(CommandError::DataProviderNotExist(repo));
  }
  let provider = provider.unwrap();
  let authors = provider.authors(&branch);
  if let Err(e) = authors {
    return Err(CommandError::GetAuthorError(format!("{} {}", repo, branch.name)));
  }
  Ok(authors.unwrap())
}

#[tauri::command]
pub fn branches(
  repo: String,
  data_provider: tauri::State<'_, Mutex<HashMap<String, GitDataProvider>>>
) -> Result<Vec<Branch>, CommandError> {
  let mut provider = data_provider.lock().unwrap();
  let provider = provider.get(&repo);
  if provider.is_none() {
    return Err(CommandError::DataProviderNotExist(repo));
  }
  let provider = provider.unwrap();
  let branches = provider.branches();
  if let Err(e) = branches {
    return Err(CommandError::BranchNotFound(repo));
  }
  Ok(branches.unwrap())
}