use anyhow::Result;
use git2::{Commit as Git2Commit, Config, Delta, Oid, Repository};

use crate::types::commit::Commit;
use crate::types::file::File;
use crate::types::status::FileStatus;

pub fn has_git() -> bool {
    if let Err(_) = std::process::Command::new("git").arg("--version").output() {
        return false; // git 命令运行失败，说明没有安装 git
    }
    true
}

pub fn build_commit(commit: &Git2Commit, repo: String) -> Commit {
    let message = if let Some(message) = commit.message() {
        message.to_string()
    } else {
        "".to_string()
    };
    let committer = commit.committer();
    let author = commit.author();
    Commit::new(
        commit.id().to_string(),
        author.name().unwrap_or("").to_string(),
        author.email().unwrap_or("").to_string(),
        committer.name().unwrap_or("").to_string(),
        committer.email().unwrap_or("").to_string(),
        message.lines().next().unwrap_or("").to_string(),
        message,
        commit.time().seconds(),
        commit.parent_ids().into_iter().count() as i64,
        repo,
    )
}

pub fn change_status_to_file_status(change: &Delta) -> FileStatus {
    match change {
        &Delta::Added => FileStatus::Added,
        &Delta::Deleted => FileStatus::Deleted,
        &Delta::Modified => FileStatus::Modified,
        &Delta::Renamed => FileStatus::Renamed,
        _ => FileStatus::Ok,
    }
}

pub fn get_blob_size(repo: &Repository, id: impl Into<Oid>) -> (bool, usize) {
    let blob = repo.find_blob(id.into());
    if let Err(_) = blob {
        return (false, 0);
    } else {
        return (true, blob.unwrap().content().len());
    }
}

pub fn build_file_between_tree(
    repo: &Repository,
    old_tree: &git2::Tree,
    new_tree: &git2::Tree,
) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    let diff = repo.diff_tree_to_tree(Some(old_tree), Some(new_tree), None);
    match diff {
        Ok(diff) => {
            for delta in diff.deltas() {
                // 文件夹特殊处理
                let new_id = delta.new_file().id();
                let old_id = delta.old_file().id();
                let status = change_status_to_file_status(&delta.status());
                let new_blob = repo.find_blob(new_id);
                let (exist, content) = match new_blob {
                    Ok(blob) => (true, blob.content().to_vec()),
                    Err(_) => (false, Vec::new()),
                };
                let size = content.len();
                let path = match delta.new_file().path() {
                    Some(path) => path.to_str().unwrap_or("").to_string(),
                    None => "".to_string(),
                };
                let file = File::new(
                    path,
                    size,
                    status,
                    new_id.to_string(),
                    old_id.to_string(),
                    exist,
                );
                files.push(file);
            }
        }
        Err(_) => {}
    }
    files
}

/// 判断是否是git仓库
pub fn is_git_repo(path: &str) -> bool {
    if let Ok(repo) = Repository::open(path) {
        return true;
    } else {
        return false;
    }
}

pub fn has_owner(path: &str) -> Result<bool, git2::Error> {
    match Repository::open(path) {
        Ok(_) => Ok(true),
        Err(err) => {
            if err.code() == git2::ErrorCode::Owner {
                return Ok(false);
            } else {
                Err(err)
            }
        }
    }
}

/// 设置全局配置，将仓库设置为可信
pub fn set_owner(path: &str) -> Result<bool, git2::Error> {
    match has_owner(&path) {
        Ok(ret) => {
            if !ret {
                let path = path.replace("\\", "/");
                let mut config = Config::open_default().unwrap();
                let ret = config.set_multivar("safe.directory", "$^", &path);
                if ret.is_err() {
                    return Err(ret.err().unwrap());
                }
                return Ok(true);
            }
            Ok(true)
        }
        Err(_) => todo!(),
    }
}
