use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::{anyhow, Result};
use git2::{Commit as Git2Commit, Config, Delta, Oid, Repository};
use chrono::{Utc, TimeZone};
use serde::Deserialize;
use serde_json::Value;

use crate::types::author::Author;
use crate::types::commit::Commit;
use crate::types::error::GitUtilsErrorCode;
use crate::types::file::CommittedFile;
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
) -> Vec<CommittedFile> {
    let mut files: Vec<CommittedFile> = Vec::new();
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
                let file = CommittedFile::new(
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
    match Repository::open(path) {
        Ok(_) => true,
        Err(e) => {
            if e.code() == git2::ErrorCode::Owner {
               return true
            }
            false
        }, 
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


// 判断文件是否为二进制文件(直接输入文件内容)
/// 返回 `Ok(true)` 表示是二进制文件，`Ok(false)` 表示是文本文件
pub fn is_binary_file_content(content: Vec<u8>) -> bool {
    
    // 读取前 1024 字节用于检测（可根据需要调整）
    let head_len = if content.len() < 1024 { content.len() } else { 1024 };
    let head = content[..head_len].to_vec();

    // 空字节检查
    if head.contains(&0x00) {
        return true;
    }

    // 统计不可打印的 ASCII 字符数量
    let non_printable_count = content.iter().filter(|&&byte| {
        !matches!(
            byte,
            0x09 | 0x0A | 0x0D |
            0x20 ..= 0xFF
        )
    }).count();

    let non_printable = content.iter().filter(|&&byte| {
        !matches!(
            byte,
            0x09 | 0x0A | 0x0D |
            0x20 ..= 0xFF
        )
    });

    // 如果不可打印字符超过 5%，视为二进制文件
    let threshold = content.len() / 20; // 5%
    if non_printable_count > threshold {
        println!("{:?}", non_printable);
        
    }
    non_printable_count > threshold
}

// 判断文件是否为二进制文件
/// 返回 `Ok(true)` 表示是二进制文件，`Ok(false)` 表示是文本文件
pub fn is_binary_file<P: AsRef<Path>>(path: P) -> std::io::Result<bool> {
    // 打开文件
    let mut file = std::fs::File::open(path)?;
    
    // 读取前 1024 字节用于检测（可根据需要调整）
    let mut buffer = [0; 1024];
    let bytes_read = file.read(&mut buffer)?;
    let content = &buffer[..bytes_read];

    let ret = is_binary_file_content(content.to_vec());
    Ok(ret)
}

pub fn get_file_content<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content);
    Ok(content) 
}

pub fn write_file<P: AsRef<Path>>(path: P, content: &[u8]) -> Result<()> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(content);
    Ok(()) 
}

pub fn stamp_to_ymd(stamp: i64) -> Result<String, String> {
    // 将时间戳转换为 DateTime<Utc> 类型
    let datetime  = Utc.timestamp_opt(stamp, 0);
    let t = match datetime {
        chrono::offset::LocalResult::Single(time) => Ok(time),
        chrono::offset::LocalResult::Ambiguous(early, last) => Ok(last),
        chrono::offset::LocalResult::None => Err("Invalid timestamp".to_string()),
    };
    let datetime = t?;
    Ok(datetime.format("%Y-%m-%d").to_string())
}

/// 判断日期是否为YYYY-MM-DD格式
pub fn valid_date(data: &str) -> bool {
    let date_regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    date_regex.is_match(data)
}


fn get_git_config(key: &str) -> Result<String, String> {
    let output = std::process::Command::new("git")
        .args(["config", "--get", key])
        .output()
        .map_err(|e| format!("Failed to execute git command: {e}"))?;

    if output.status.success() {
        let value = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
        
        if value.is_empty() {
            Err(format!("Git config '{key}' exists but is empty"))
        } else {
            Ok(value)
        }
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "Git config '{key}' not found or error occurred: {}",
            error_msg.trim().trim_matches(&['\n', '\r'][..])
        ))
    }
}

pub fn get_global_git_author() -> Result<Author, String> {
    let name = get_git_config("user.name")?;
    let email = get_git_config("user.email")?;
    Ok(Author::new(name, email))
}

pub fn deserialize_from_map<T>(map: &HashMap<String, Value>, key: &str, default: T) -> T
where
    T: for<'a> Deserialize<'a>,
{
    // 从 HashMap 中获取 value
    let value = map.get(key);
    if value.is_none() {
        return default;
    }
    let value = value.unwrap();
    // 将 value 反序列化为目标类型
    serde_json::from_value(value.clone()).unwrap_or(default)
}

pub fn size_by_path<P: AsRef<Path>>(path: P) -> Result<u64> {
    let path = path.as_ref();
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len()) 
}
pub fn str_to_oid(str: &str) -> Result<Oid, GitUtilsErrorCode> {
    let oid = Oid::from_str(str);
    match oid {
        Ok(oid) => Ok(oid),
        Err(_) => Err(GitUtilsErrorCode::OtherError(format!("invalid object id: {}", str))), 
    }
}

pub fn time_to_ymd(stamp: i64) -> Result<String> {
    // 将时间戳转换为 DateTime<Utc> 类型
    let datetime  = Utc.timestamp_opt(stamp, 0); 
    let t = match datetime {
        chrono::offset::LocalResult::Single(time) => Ok(time),
        chrono::offset::LocalResult::Ambiguous(early, last) => Ok(last),
        chrono::offset::LocalResult::None => Err(anyhow!("Invalid timestamp".to_string())), 
    };
    let datetime = t?;
    Ok(datetime.format("%Y-%m-%d").to_string())
}