use std::collections::HashMap;

use giter_utils::types::author::Author;
use serde::{Deserialize, Serialize};

type RepoAlias = String;
pub type RepoPath = String;
pub type BranchName = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorCache {
    pub authors: Option<Vec<Author>>,
    pub latest_commit_id: Option<String>,
}

pub type BranchAuthorCache = HashMap<BranchName, AuthorCache>;
