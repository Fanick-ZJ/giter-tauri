use std::io::Read;

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

pub fn build_commit(commit: &Git2Commit, repo: &Repository) -> Commit {
    let message = if let Some(message) = commit.message() {
        message.to_string()
    } else {
        "".to_string()
    };
    let committer = commit.committer();
    let author = commit.author();
    let mut parents = commit.parent_ids().into_iter().collect::<Vec<Oid>>();
    parents.sort_by(|a, b| {
        repo.find_commit(*b).unwrap().time().seconds().cmp(&repo.find_commit(*a).unwrap().time().seconds())
    });
    let path = repo.workdir().unwrap().to_str().unwrap().to_string();
    Commit::new(
        commit.id().to_string(),
        author.name().unwrap_or("").to_string(),
        author.email().unwrap_or("").to_string(),
        committer.name().unwrap_or("").to_string(),
        committer.email().unwrap_or("").to_string(),
        message.lines().next().unwrap_or("").to_string(),
        message,
        commit.time().seconds() * 1000,
        parents,
        path,
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
                let is_binary = delta.new_file().is_binary();
                let old_is_binary = delta.old_file().is_binary();
                let file = File::new(
                    path,
                    size,
                    status,
                    new_id.to_string(),
                    old_id.to_string(),
                    exist,
                    is_binary,
                    old_is_binary,
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


// 判断文件是否为二进制文件
/// 返回 `Ok(true)` 表示是二进制文件，`Ok(false)` 表示是文本文件
pub fn is_binary_file(path: &str) -> std::io::Result<bool> {
    // 打开文件
    let mut file = std::fs::File::open(path)?;
    
    // 读取前 1024 字节用于检测（可根据需要调整）
    let mut buffer = [0; 1024];
    let bytes_read = file.read(&mut buffer)?;
    let content = &buffer[..bytes_read];

    // 空字节检查
    if content.contains(&0x00) {
        return Ok(true);
    }

    // 统计不可打印的 ASCII 字符数量
    let non_printable_count = content.iter().filter(|&&byte| {
        // 允许的字符：制表符（\t）、换行（\n）、回车（\r）以及可打印 ASCII
        !(byte == 0x09 || 
          byte == 0x0A || 
          byte == 0x0D || 
          (byte >= 0x20 && byte <= 0x7E))
    }).count();

    // 如果不可打印字符超过 5%，视为二进制文件
    let threshold = content.len() / 20; // 5%
    Ok(non_printable_count > threshold)
}