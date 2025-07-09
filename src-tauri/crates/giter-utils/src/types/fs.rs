use std::path::{PathBuf};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum EntryMode {
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

impl From<i32> for EntryMode {
    fn from(value: i32) -> Self {
        return match value {
            0o000000 => EntryMode::Unreadable,
            0o040000 => EntryMode::Tree,
            0o100644 => EntryMode::Blob,
            0o100664 => EntryMode::BlobGroupWritable,
            0o100755 => EntryMode::BlobExecutable,
            0o120000 => EntryMode::Link,
            0o160000 => EntryMode::Commit,
            _ => EntryMode::Unreadable
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EntryMetadata {
    pub size: usize,
    pub mode: EntryMode,
    pub object_id: String,
}
#[derive(Debug, Serialize)]
pub struct File {
    pub name: String,
    pub path: String,
    pub metadata: EntryMetadata,
}

#[derive(Debug, Serialize)]
pub struct Dir {
    pub path: String,
    pub name: String,
    pub children: Vec<FsNode>,
    pub metadata: EntryMetadata,
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
    pub fn new(path: String, name: String, object_id: String, size: usize, mode: EntryMode) -> Self {
        File {
            name,
            path,
            metadata: EntryMetadata {
                size,
                object_id,
                mode
            },
        }
    }
}

impl Dir {
    pub fn new(path: String, name: String, object_id: String) -> Self {
        Dir {
            path,
            name,
            children: Vec::new(),
            metadata: EntryMetadata { size: 0, mode: EntryMode::Tree, object_id }
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