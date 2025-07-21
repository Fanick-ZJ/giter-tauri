use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::util::valid_date;

use super::{author::Author, branch::Branch};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommitStatistic {
    pub repo: PathBuf,
    pub branch: Branch,
    pub author: Author,
    pub stats: HashMap<String, i32>,
}

impl CommitStatistic {
    pub fn new(repo: PathBuf, branch: Branch, author: Author) -> Self {
        Self {
            repo,
            branch,
            author,
            stats: HashMap::new(),
        }
    }

    pub fn add(&mut self, time: String, count: i32) -> Result<(), String> {
        if !valid_date(&time) {
            return Err(format!("invalid date: {}", time));
        }
        let old_value = self.stats.get(&time).unwrap_or(&0);
        self.stats.insert(time, old_value + count);
        Ok(())
    }
}
