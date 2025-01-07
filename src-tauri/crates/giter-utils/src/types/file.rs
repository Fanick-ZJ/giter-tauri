use gix::objs::tree::EntryKind;
use serde::{Deserialize, Serialize};
use types::status::FileStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
  path: String,
  size: usize,
  status: FileStatus,
  object_id: String,
  entry_kind: EntryKind,
}

impl File {
  pub fn new(path: String, size: usize, status: FileStatus, object_id: String, entry_kind: EntryKind) -> Self {
    Self {
      path,
      size,
      status,
      object_id,
      entry_kind
    }
  }
}