use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(transparent)] // 或者 `untagged` 也行，因为只有一个字段
pub struct EntryMode(u32);

impl EntryMode {
    pub const BLOB: EntryMode = EntryMode(0o100644);
    pub const BLOB_EXECUTABLE: EntryMode = EntryMode(0o100755);
    pub const LINK: EntryMode = EntryMode(0o120000);
    pub const COMMIT: EntryMode = EntryMode(0o160000);
    pub const TREE: EntryMode = EntryMode(0o040000);
    pub const UNREADABLE: EntryMode = EntryMode(0o000000);
    pub const BLOB_GROUP_WRITABLE: EntryMode = EntryMode(0o100664);
}

impl From<i32> for EntryMode {
    fn from(value: i32) -> Self {
        return match value {
            0o000000 => EntryMode::UNREADABLE,
            0o040000 => EntryMode::TREE,
            0o100644 => EntryMode::BLOB,
            0o100664 => EntryMode::BLOB_GROUP_WRITABLE,
            0o100755 => EntryMode::BLOB_EXECUTABLE,
            0o120000 => EntryMode::LINK,
            0o160000 => EntryMode::COMMIT,
            _ => EntryMode::UNREADABLE,
        };
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
    Dir(Dir), // 展开目录内容
}

impl File {
    pub fn new(
        path: String,
        name: String,
        object_id: String,
        size: usize,
        mode: EntryMode,
    ) -> Self {
        let path = if path.ends_with("/") {
            path[0..path.len() - 1].to_string()
        } else {
            path
        };
        File {
            name,
            path,
            metadata: EntryMetadata {
                size,
                object_id,
                mode,
            },
        }
    }
}

/// path: 输入可以为 "a/b/c"也可为 "a/b/c/"，在新建的时候会自动去除最后一项
/// name: 就为单个单词
impl Dir {
    pub fn new(path: String, name: String, object_id: String) -> Self {
        let path = if path.ends_with("/") {
            path[0..path.len() - 1].to_string()
        } else {
            path
        };
        Dir {
            path,
            name,
            children: Vec::new(),
            metadata: EntryMetadata {
                size: 0,
                mode: EntryMode::TREE,
                object_id,
            },
        }
    }
    pub fn get_children_mut(&mut self) -> &mut Vec<FsNode> {
        &mut self.children
    }

    pub fn abs_path(&self) -> String {
        if self.path.is_empty() {
            if self.name.is_empty() {
                return "".into();
            } else {
                return format!("{}/", self.name);
            }
        } else {
            format!("{}/{}/", self.path, self.name)
        }
    }

    pub fn add(&mut self, node: FsNode) {
        self.children.push(node);
    }
}
