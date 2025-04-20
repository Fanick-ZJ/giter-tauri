use std::{fmt, path::{Path, PathBuf}};

use git2::Oid;
use serde::{
    de::{Error as DeError, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize
};
use crate::util::str_to_oid;

use super::{author::Author, commit::Commit, status::FileStatus};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommittedFile {
    pub path: String,
    pub size: usize,
    pub status: FileStatus,
    pub object_id: String,
    pub prev_object_id: String,
    pub blob_exist: bool,
    pub is_binary: bool,
    pub old_is_binary: bool,
}

impl CommittedFile {
    pub fn new(
        path: String,
        size: usize,
        status: FileStatus,
        object_id: String,
        prev_object_id: String,
        blob_exist: bool,
        is_binary: bool,
        old_is_binary: bool,
    ) -> Self {
        Self {
            path,
            size,
            status,
            object_id,
            prev_object_id,
            blob_exist,
            is_binary,
            old_is_binary,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folder {
    path: String,
    object_id: String,
}

impl Folder {
    pub fn new(path: String, object_id: String) -> Self {
        Self { path, object_id }
    }
}


#[derive(Debug, Clone)]
pub struct UntrackedFile {
    pub path: Box<Path>,
    pub size: usize,
    pub is_binary: bool
}

impl UntrackedFile {
    pub fn new<T: AsRef<Path>>(path: T, size: usize, is_binary: bool) -> Self {
        let path_buf = path.as_ref().to_path_buf();
        Self {
            path: path_buf.into_boxed_path(),
            size,
            is_binary
        }
    }
}

impl Serialize for UntrackedFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("UntrackedFile", 3)?;
        s.serialize_field("path", &self.path.to_string_lossy())?;
        s.serialize_field("size", &self.size)?;
        s.serialize_field("isBinary", &self.is_binary)?;
        s.end()
    }
}
impl<'de> Deserialize<'de> for UntrackedFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UntrackedFileVisitor;

        impl<'de> Visitor<'de> for UntrackedFileVisitor {
            type Value = UntrackedFile;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct UntrackedFile")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UntrackedFile, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut path = None;
                let mut size = None;
                let mut is_binary = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "path" => path = Some(PathBuf::from(map.next_value::<String>()?).into_boxed_path()),
                        "size" => size = Some(map.next_value()?),
                        "isBinary" => is_binary = Some(map.next_value()?),
                        _ => (),
                    }
                }

                Ok(UntrackedFile {
                    path: path.ok_or_else(|| DeError::missing_field("path"))?.into(),
                    size: size.ok_or_else(|| DeError::missing_field("size"))?,
                    is_binary: is_binary.ok_or_else(|| DeError::missing_field("isBinary"))?,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "path",
            "size",
            "isBinary",
        ];

        deserializer.deserialize_struct("UntrackedFile", FIELDS, UntrackedFileVisitor)
    }
}
 



#[derive(Debug, Clone)]
pub struct ChangedFile {
    pub path: Box<Path>,
    pub prev_object_id: Oid,
    pub status: FileStatus,
}

impl ChangedFile {
    pub fn new<T: AsRef<Path>>(
        path: T,
        prev_object_id: Oid,
        status: FileStatus,
    ) -> Self {
        let path_buf = path.as_ref().to_path_buf();
        Self {
            path: path_buf.into_boxed_path(),
            prev_object_id,
            status,
        }
    }
}

impl Serialize for ChangedFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ChangedFile", 3)?;
        s.serialize_field("path", &self.path.to_string_lossy())?;
        s.serialize_field("prevObjectId", &self.prev_object_id.to_string())?;
        s.serialize_field("status", &self.status)?;
        s.end()
    }
}



#[derive(Debug, Clone)]
pub struct FileHistoryEntry {
    /// 文件在该提交中的状态
    pub file: CommittedFile,
    pub commit: Commit
}

impl Serialize for FileHistoryEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("FileHistoryEntry", 2)?;
        s.serialize_field("commit", &self.commit)?; 
        s.serialize_field("file", &self.file)?;
        s.end()
    } 
}

impl<'de> Deserialize<'de> for FileHistoryEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitVisitor;

        impl<'de> Visitor<'de> for CommitVisitor {
            type Value = FileHistoryEntry;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct FileHistoryEntry")
            }

            fn visit_map<V>(self, mut map: V) -> Result<FileHistoryEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut commit = None;
                let mut file = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "commit" => commit = Some(map.next_value()?),
                        "file" => file = Some(map.next_value()?),
                        _ => (),
                    }
                }

                Ok(FileHistoryEntry {
                    commit: commit.ok_or_else(|| DeError::missing_field("commit"))?,
                    file: file.ok_or_else(|| DeError::missing_field("file"))?,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "commitId",
            "file",
            "commitDate",
            "author",
            "message",
        ];

        deserializer.deserialize_struct("FileHistory", FIELDS, CommitVisitor)
    }
}

impl FileHistoryEntry {
    pub fn new(
        commit: Commit,
        file: CommittedFile,
    ) -> Self {
        Self {
            commit,
            file,
        }
    }
}