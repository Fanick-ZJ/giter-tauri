use std::{collections::HashMap, sync::Mutex};
use giter_utils::types::{author::Author, branch::Branch, git_data_provider::GitDataProvider};
use crate::types::error::CommandError;

fn get_provider<'a>(
  repo: &str,
  data_provider: &'a HashMap<String, GitDataProvider>
) -> Result<&'a GitDataProvider, CommandError> {
  // let provider = data_provider.lock().unwrap();
  data_provider.get(repo).ok_or(CommandError::DataProviderNotExist(repo.to_string()))
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