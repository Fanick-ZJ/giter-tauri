use std::{fs::File, io::Write, path::{Path, PathBuf}, time};

use giter_utils::{types::{error::GitUtilsErrorCode, git_data_provider::GitDataProvider}, util::str_to_oid};
use serde_json::{json, to_string_pretty};

#[test]
fn test_get_commit_file_tree() -> Result<(), GitUtilsErrorCode> {
    let path = Path::new(r"E:\workSpace\linux");
    let oid = str_to_oid("7cdabafc001202de9984f22c973305f424e0a8b7")?;
    let repo = GitDataProvider::new(path)?;
    let t1 = time::Instant::now();
    let tree = repo.get_tree_recursive(oid, None)?;
    let mut file = File::create(r"C:\Users\ZJFan\OneDrive\桌面\1.json")?;
    file.write_all(to_string_pretty(&json!(tree)).unwrap().as_bytes());
    let t2 = time::Instant::now();
    eprintln!("cost time: {:?} ms", t2 - t1);
    Ok(())
}

#[test]
fn test_get_commit_file_tree_by_level() -> Result<(), GitUtilsErrorCode> {
    let path = Path::new(r"E:\workSpace\linux");
    let oid = str_to_oid("7cdabafc001202de9984f22c973305f424e0a8b7")?;
    let repo = GitDataProvider::new(path)?;
    let tree = repo.get_tree(oid, None)?;
    let mut file = File::create(r"C:\Users\ZJFan\OneDrive\桌面\1.json")?;
    file.write_all(to_string_pretty(&json!(tree)).unwrap().as_bytes());
    Ok(())
}

#[test]
fn test_get_tree_recursive() -> Result<(), GitUtilsErrorCode> {
    let path = Path::new(r"E:\workSpace\linux");
    let oid = str_to_oid("4ca54b871ae1dd229e2c917f6b1e72300cb3aaf1")?;
    let repo = GitDataProvider::new(path)?;
    let tree = repo.get_tree_recursive(oid, Some("arch/alpha/boot".into()))?;
    let mut file = File::create(r"C:\Users\ZJFan\OneDrive\桌面\1.json")?;
    file.write_all(to_string_pretty(&json!(tree)).unwrap().as_bytes());
    Ok(())
}

#[test]
fn test_path() {
    let path = PathBuf::from("crates/tauri-macos-sign/src/keychain/");
    println!("{:?}", path.join("tauri-macros/"))
}