use std::path::{Path, PathBuf};

use giter_utils::{types::{error::GitUtilsErrorCode, git_data_provider::GitDataProvider}, util::str_to_oid};

#[test]
fn test_get_commit_file_tree() -> Result<(), GitUtilsErrorCode> {
    let path = Path::new("E:\\workSpace\\Rust\\tauri");
    let oid = str_to_oid("85b19125294917e10e89fc9e09722eaaa4f69962")?;
    let repo = GitDataProvider::new(path)?;
    let tree = repo.get_commit_file_tree(oid);

    Ok(())
}

#[test]
fn test_path() {
    let path = PathBuf::from("crates/tauri-macos-sign/src/keychain/");
    println!("{:?}", path.join("tauri-macros/"))
}