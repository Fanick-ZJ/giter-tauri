use std::path::PathBuf;

use anyhow::Result;

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
}
