use std::{path::{Path, PathBuf}, time};

use giter_utils::{types::{error::GitUtilsErrorCode, git_data_provider::GitDataProvider}, util::str_to_oid};

#[test]
fn test_get_commit_file_tree() -> Result<(), GitUtilsErrorCode> {
    let path = Path::new(r"E:\workSpace\linux");
    let oid = str_to_oid("7cdabafc001202de9984f22c973305f424e0a8b7")?;
    let repo = GitDataProvider::new(path)?;
    let t1 = time::Instant::now();
    let tree = repo.get_commit_file_tree(oid);
    let t2 = time::Instant::now();
    eprintln!("cost time: {:?} ms", t2 - t1);
    Ok(())
}

#[test]
fn test_path() {
    let path = PathBuf::from("crates/tauri-macos-sign/src/keychain/");
    println!("{:?}", path.join("tauri-macros/"))
}