use std::io::Read;
use std::process::Command;
use std::time::UNIX_EPOCH;

use anyhow::Result;
use git2::{Commit as Git2Commit, Config, Delta, Oid, Repository};

use crate::types::author::Author;
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

// 简化的日期计算函数（仅作演示）
fn calculate_date(days: u64) -> (u64, u64, u64) {
    let days_in_year = 365;
    let year = 1970 + days / days_in_year;
    let remaining_days = days % days_in_year;
    let month = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    (year, month, day)
}

pub fn second_to_date(second: i64) -> Result<String, String> {
    let timestamp = UNIX_EPOCH + std::time::Duration::from_secs(second as u64);
    match timestamp.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let seconds = duration.as_secs();
            let minutes = seconds / 60;
            let hours = minutes / 60;
            let days = hours / 24;
            let (year, month, day) = calculate_date(days as u64);
            // month 和 day 都需要补零
            let month = if month < 10 { format!("0{}", month) } else { format!("{}", month) };
            let day = if day < 10 { format!("0{}", day) } else { format!("{}", day) };
            return Ok(format!("{}-{}-{}", year, month, day));
        }
        Err(_) => {
            return Err("Invalid timestamp".to_string());
        }
    }
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

