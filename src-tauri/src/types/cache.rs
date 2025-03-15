use std::collections::HashMap;

use giter_utils::types::author::Author;
use serde::{Deserialize, Serialize};

pub type RepoPath = String;
pub type BranchName = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorCache {
    pub authors: Option<Vec<Author>>,
    pub last_commit_id: Option<String>,
}

pub type BranchAuthorCache = HashMap<BranchName, AuthorCache>;
