use serde::{Deserialize, Serialize};
use types::status::FileStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub path: String,
    pub size: usize,
    pub status: FileStatus,
    pub object_id: String,
    pub prev_object_id: String,
    pub blob_exist: bool,
    pub is_binary: bool,
    pub old_is_binary: bool,
}

impl File {
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
