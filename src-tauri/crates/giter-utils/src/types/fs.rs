use std::path::{PathBuf};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum FileMode {
    #[serde(rename = "blob")]
    Blob = 0o100644,
    #[serde(rename = "blob_executable")]
    BlobExecutable = 0o100755,
    #[serde(rename = "link")]
    Link = 0o120000,
    #[serde(rename = "commit")]
    Commit = 0o160000,
    #[serde(rename = "tree")]
    Tree = 0o040000,
    #[serde(rename = "unreadable")]
    Unreadable = 0o000000,
    #[serde(rename = "blob_group_writable")]
    BlobGroupWritable = 0o100664
}

impl From<i32> for FileMode {
    fn from(value: i32) -> Self {
        return match value {
            0o000000 => FileMode::Unreadable,
            0o040000 => FileMode::Tree,
            0o100644 => FileMode::Blob,
            0o100664 => FileMode::BlobGroupWritable,
            0o100755 => FileMode::BlobExecutable,
            0o120000 => FileMode::Link,
            0o160000 => FileMode::Commit,
            _ => FileMode::Unreadable
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FileMetadata {
    pub size: usize,
    pub file_mode: FileMode,
}
#[derive(Debug, Serialize)]
pub struct File {
    pub name: String,
    pub path: String,
    pub metadata: FileMetadata,
}

#[derive(Debug, Serialize)]
pub struct Dir {
    pub path: String,
    pub name: String,
    pub children: Vec<FsNode>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")] // 仅保留类型标签
pub enum FsNode {
    #[serde(rename = "file")]
    File(File), // 展开文件内容
    
    #[serde(rename = "dir")]
    Dir(Dir),   // 展开目录内容
}

impl File {
    pub fn new_file(path: String, name: String, size: usize, file_mode: FileMode) -> Self {
        File {
            name,
            path,
            metadata: FileMetadata {
                size,
                file_mode
            },
        }
    }
}

impl Dir {
    pub fn new_dir(path: String, name: String) -> Self {
        Dir {
            path,
            name,
            children: Vec::new(),
        }
    }
    pub fn get_children_mut(&mut self) -> &mut Vec<FsNode> {
        &mut self.children
    }

    pub fn abs_path(&self) -> PathBuf {
        if self.path.is_empty() && self.name.is_empty() {
            return PathBuf::from("")
        }
        let path_buf = PathBuf::from(&self.path);
        return path_buf.join(self.name.to_string() + "/")
    }

    pub fn add(&mut self, node: FsNode) {
        self.children.push(node);
    }
}