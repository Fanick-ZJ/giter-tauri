use std::any::Any;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use gix::diff::tree_with_rewrites::Change;
use gix::objs::tree::{EntryKind, EntryMode};
use gix::{self, Repository};

pub fn validate_git_repository(repository: &str) -> Result<gix::Repository, String> {
    let git_repository = gix::open(repository);
    if git_repository.is_err() {
        return Err(git_repository.unwrap_err().to_string());
    }
    let repository = git_repository.ok().unwrap();
    Ok(repository)
}

pub fn has_git() -> bool {
    if let Err(_) = std::process::Command::new("git").arg("--version").output() {
        return false; // git 命令运行失败，说明没有安装 git
    }
    true
}

pub fn string_to_object_id(id: String) -> Result<gix::ObjectId> {
    Ok(gix::ObjectId::from_str(id.as_str())?)
}

use crate::types::commit::Commit;
use crate::types::file::File;
use crate::types::status::FileStatus;

pub fn build_commit(commit: &gix::Commit) -> Commit {
    let message = if let Some(body) = commit.message().unwrap().body() {
        body.to_string()
    } else {
        "".to_string()
    };
    Commit::new(
        commit.id.to_string(),
        commit.author().unwrap().name.to_string(),
        commit.author().unwrap().email.to_string(),
        commit.committer().unwrap().name.to_string(),
        commit.committer().unwrap().email.to_string(),
        commit.message().unwrap().summary().to_string(),
        message,
        commit.time().unwrap().seconds as i64,
        commit.parent_ids().into_iter().count() as i64,
        commit.repo.path().to_str().unwrap().to_string(),
    )
}

fn change_status_to_fiel_status(change: &Change) -> FileStatus {
    match change {
        Change::Addition { .. } => FileStatus::Added,
        Change::Deletion { .. } => FileStatus::Deleted,
        Change::Modification { .. } => FileStatus::Modified,
        Change::Rewrite { .. } => FileStatus::Renamed,
    }
}

pub fn get_blob_size(repo: &Repository, id: impl Into<gix::ObjectId>) -> (bool, usize) {
    let blob = repo.find_blob(id);
    if let Err(_) = blob {
        return (false, 0);
    } else {
        return (true, blob.unwrap().data.len() as usize);
    }
}

// 从change中构建file
pub fn build_file_from_change(repo: &Repository, change: &Change) -> Result<File> {
    if change.entry_mode().is_tree() {
        return Err(anyhow!("It's tree!"));
    }
    let (prev_entry_mod, previous_id) = change.source_entry_mode_and_id();
    let (entry_mode, id) = change.entry_mode_and_id();
    let status = change_status_to_fiel_status(change);
    let (exist, size) = get_blob_size(repo, id);
    let location = change.location().to_string();
    change.source_location();
    let file = File::new(
        location,
        size,
        status,
        id.to_string(),
        EntryKind::from(entry_mode),
        previous_id.to_string(),
        exist,
    );
    Ok(file)
}

pub fn build_file_between_tree(
    repo: &Repository,
    old_tree: &gix::Tree,
    new_tree: &gix::Tree,
) -> Vec<(File)> {
    let mut files: Vec<File> = Vec::new();
    let diff = repo.diff_tree_to_tree(old_tree, new_tree, gix::diff::Options::default());
    match diff {
        Ok(changes) => {
            for change in changes {
                // println!("-----{:?}", change.location().to_string());
                // 文件夹特殊处理
                if change.entry_mode().is_tree() {
                    continue;
                } else {
                    let file = build_file_from_change(repo, &change);
                    match file {
                        Ok(file) => {
                            files.push(file);
                        }
                        Err(_) => {
                            println!("build file error");
                        }
                    }
                }
            }
        }
        Err(_) => {}
    }
    files
}
