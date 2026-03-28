use std::path::PathBuf;

use anyhow::Result;
use git2::Repository;

use windows::Win32::Storage::FileSystem::*;

use crate::types::fs::{Catalog, Dir, File};
use giter_utils::util::is_git_repo;

pub fn read_json_file<T>(file_path: impl Into<PathBuf>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let file = std::fs::File::open(file_path.into())?;
    let reader = std::io::BufReader::new(file);
    let data = serde_json::from_reader::<_, T>(reader)?;
    Ok(data)
}

/// 获取所有盘符
/// 返回值为盘符的字符串数组
/// 例如：["C:\\", "D:\\"]
pub fn get_logical_driver() -> Vec<String> {
    // 第一次返回的是buffer的长度
    let driveer_length = unsafe { GetLogicalDriveStringsA(None) };
    // 根据长度创建buffer
    let mut buffer = vec![0u8; driveer_length as usize];
    // &mut buffer[..] 意思是将buffer的引用
    unsafe { GetLogicalDriveStringsA(Some(&mut buffer[..])) };
    let s = String::from_utf8_lossy(&buffer);
    s.split("\0")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// 获取指定目录下的第一级子目录
///
pub fn get_first_level_dirs(dir_path: &str) -> Result<Catalog> {
    // 判断是否是目录
    if !std::path::Path::new(dir_path).is_dir() {
        return Err(anyhow::anyhow!("{} is not a directory", dir_path));
    }
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    let dir = std::fs::read_dir(dir_path)?;
    for entry in dir {
        let path_buf = entry?.path();
        let path = path_buf.to_str().unwrap().to_string();
        let name = path.split("\\").last().unwrap().to_string();
        if path_buf.is_dir() {
            let is_repo = is_git_repo(&path);
            dirs.push(Dir::new(name, path, is_repo));
        } else {
            let size = std::fs::metadata(&path)?.len();
            files.push(File::new(name, path, size));
        }
    }
    Ok(Catalog::new(dirs, files))
}

/// 检查一个目录是否是某个仓库的子模块（submodule）
///
/// # Arguments
/// * `parent_path` - 可能的父仓库路径
/// * `submodule_path` - 要检查的子模块路径
///
/// # Returns
/// * `true` - 是子模块
/// * `false` - 不是子模块
pub fn is_submodule(parent_path: &str, submodule_path: &str) -> bool {
    // 首先检查父路径是否是一个有效的仓库
    let parent_repo = match Repository::open(parent_path) {
        Ok(repo) => repo,
        Err(_) => return false,
    };

    // 检查是否有 .gitmodules 文件
    let workdir = match parent_repo.workdir() {
        Some(dir) => dir,
        None => return false,
    };

    let gitmodules_path = workdir.join(".gitmodules");
    if !gitmodules_path.exists() {
        return false;
    }

    // 读取并解析 .gitmodules 文件
    let content = match std::fs::read_to_string(&gitmodules_path) {
        Ok(content) => content,
        Err(_) => return false,
    };

    // 检查子模块路径是否在 .gitmodules 中
    let submodule_normalized = submodule_path.replace("\\", "/");
    let parent_normalized = parent_path.replace("\\", "/");

    // 如果 submodule_path 是 parent_path 的子目录，提取相对路径
    let relative_path = if submodule_normalized.starts_with(&parent_normalized) {
        let rest = &submodule_normalized[parent_normalized.len()..];
        // 移除开头的斜杠
        if rest.starts_with('/') {
            rest[1..].to_string()
        } else {
            rest.to_string()
        }
    } else {
        return false;
    };

    // 在 .gitmodules 中查找路径
    content
        .lines()
        .filter(|line| line.starts_with("path = "))
        .any(|line| {
            let path = line.trim_start_matches("path = ").trim();
            path == &relative_path || path == &submodule_normalized
        })
}

/// 扫描指定文件夹下的所有 Git 仓库，不包括子模块
///
/// # Arguments
/// * `folder_path` - 要扫描的文件夹路径
///
/// # Returns
/// 成功返回仓库路径列表，失败返回错误信息
pub fn scan_repos_in_folder(folder_path: &str) -> Result<Vec<String>> {
    let path = std::path::Path::new(folder_path);

    // 检查是否是目录
    if !path.is_dir() {
        return Err(anyhow::anyhow!("{} is not a directory", folder_path));
    }

    let mut repos = Vec::new();
    let entries = std::fs::read_dir(folder_path)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        // 只处理目录
        if !entry_path.is_dir() {
            continue;
        }

        let path_str = entry_path.to_str().unwrap().to_string();

        // 检查是否是 Git 仓库
        if is_git_repo(&path_str) {
            // 检查是否是子模块
            // 先检查是否有父仓库包含这个子模块
            let is_sub = is_submodule(folder_path, &path_str);

            if !is_sub {
                repos.push(path_str);
            }
        }
    }

    Ok(repos)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_logical_driver() {
        let logical_drivers = get_logical_driver();
        println!(";;;;;;;;;;{:?}", logical_drivers)
    }

    #[test]
    fn test_get_first_level_dirs() {
        let dirs = get_first_level_dirs("E:\\workSpace\\Python_Project_File");
        if dirs.is_ok() {
            let dirs = dirs.unwrap();
            for dir in dirs.dirs {
                println!("{:?}", dir);
            }
            for file in dirs.files {
                println!("{:?}", file);
            }
        }
    }

    #[test]
    fn test_scan_repos_in_folder() {
        match scan_repos_in_folder("E:\\workSpace\\JavaScript") {
            Ok(repos) => {
                println!("Found repos:");
                for repo in &repos {
                    println!("  - {}", repo);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_is_submodule() {
        // 测试子模块检测
        let parent = "E:\\workSpace\\JavaScript\\some-parent-repo";
        let submodule = "E:\\workSpace\\JavaScript\\some-parent-repo\\submodule";
        let result = is_submodule(parent, submodule);
        println!("Is submodule: {}", result);
    }
}
