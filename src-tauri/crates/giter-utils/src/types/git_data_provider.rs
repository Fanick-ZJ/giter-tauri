use crate::util::change_status_to_file_status;
use crate::util::build_commit;
use anyhow::Result;
use git2::{BranchType, Config, ErrorCode, Oid, Repository, Revwalk, Status};
use log::error;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Pointer;
use std::path;
use std::vec;
use types::{
  author::Author, branch::Branch, commit::Commit,
  status::WorkStatus,
};

use super::cache::Cache;
use super::file::File;
pub struct GitDataProvider {
  pub repository: Repository,
  cache: RefCell<Option<Box<dyn Cache + Send>>>
}

impl Pointer for GitDataProvider {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("GitDataProvider")
      .field(&format!("{:p}", &self.repository).to_string(), &"".to_string())
      .finish()
  }
}

impl GitDataProvider {
  pub fn new(repository: &str) -> Result<Self, git2::Error> {
    let repo = Repository::open(repository);
    match repo {
      Ok(repo) => Ok(GitDataProvider { repository: repo, cache: RefCell::new(None) }),
      Err(err) => {
        log::error!("{}", err);
        Err(err)
      },
    }
  }

  pub fn set_cache(&mut self, cache: impl Cache + Send + 'static) {
    self.cache = RefCell::new(Some(Box::new(cache)));
  }

  pub fn workdir(&self) -> String {
    self.repository.workdir().unwrap().to_str().unwrap().to_string() 
  }

  /// 获取位追踪的文件列表
  ///
  pub fn untracked_files(&self) -> Result<Vec<String>> {
    let status = self.repository.statuses(None);
    let mut untracks: Vec<String> = Vec::new();
    match status {
      Ok(status) => {
        for item in &status {
          if item.status() == Status::WT_NEW {
            untracks.push(item.path().unwrap_or("").to_string());
          }
        }
      } 
      Err(err) => {
        return Err(anyhow::anyhow!(err.message().to_string()))
      }
    }
    Ok(untracks)
  }
  ///工作空间是否有修改
  ///
  pub fn workspace_change(&self) -> Result<Vec<String>, git2::Error> {
    let status = self.repository.statuses(None);
    let mut modified: Vec<String> = Vec::new();
    match status {
      Ok(status) => {
        for item in &status {
          let bits = item.status().bits();
          let index_status =
              Status::WT_DELETED.bits()
              | Status::WT_MODIFIED.bits()
              | Status::WT_NEW.bits()
              | Status::WT_RENAMED.bits()
              | Status::WT_TYPECHANGE.bits();
          if bits & index_status > 0 {
            modified.push(item.path().unwrap_or("").to_string());
          }
        }
        Ok(modified)
      } 
      Err(err) => {
        Err(err)
      }
    }
  }
  
  pub fn uncommitted (&self) -> Result<bool> {
    let status = self.repository.statuses(None);
    match status {
      Ok(status) => {
        for item in &status {
          let bits = item.status().bits();
          let index_status = 
              Status::INDEX_DELETED.bits() 
              | Status::INDEX_MODIFIED.bits() 
              | Status::INDEX_NEW.bits() 
              | Status::INDEX_RENAMED.bits() 
              | Status::INDEX_TYPECHANGE.bits();
          if (bits & index_status)  > 0 {
            return Ok(true);
          }
        }
      } 
      Err(err) => {
        return Err(anyhow::anyhow!(err.message().to_string()))
      }
    }
    Ok(false)
  }

  pub fn current_branch(&self) -> Result<Branch> {
    let repo = &self.repository;
    let branches = repo.branches(None).unwrap();
    for branch in branches {
      let (branch, branch_type) = branch.unwrap();
      if branch.is_head() {
        let reference = branch.name().unwrap_or(Some("")).unwrap_or("").to_string();
        let name = reference.split(path::MAIN_SEPARATOR).last().unwrap_or("").to_string();
        return Ok(Branch {
          name,
          is_remote: branch_type == BranchType::Remote,
          reference,
        });
      } 
    }
    Err(anyhow::anyhow!("No current branch"))
  }

  pub fn branches(&self) -> Result<Vec<Branch>> {
    let repo = &self.repository;
    let branches = repo.branches(None).unwrap();
    let mut _branches: Vec<Branch> = Vec::new(); 
    for branch in branches {
      let (branch, branch_type) = branch.unwrap();
      let reference = branch.name().unwrap_or(Some("")).unwrap_or("").to_string();
      let name = reference.split(path::MAIN_SEPARATOR).last().unwrap_or("").to_string(); 
      _branches.push(Branch {
        name,
        is_remote: branch_type == BranchType::Remote,
        reference, 
      })
    }
    Ok(_branches)
  }

  pub fn current_remote_branch(&self) -> Result<Branch> {
    let current = self.current_branch()?;
    let remote = self.repository.find_branch(&current.reference, BranchType::Local)?.upstream();
    match remote {
      Ok(remote) => {
        let remote_name = remote.name().unwrap_or(Some("")).unwrap().to_string();
        let reference = remote.name().unwrap_or(Some("")).unwrap().to_string();
        Ok(Branch {
          name: remote_name,
          is_remote: true,
          reference, 
        })
      }
      Err(_) => {
        Err(anyhow::anyhow!("No remote branch"))
      }
    }
  }

  fn current_remote_branch_inner(&self) -> Result<git2::Branch> {
    let current = self.current_branch()?;
    let remote = self.repository.find_branch(&current.reference, BranchType::Local)?.upstream();
    match remote {
      Ok(remote) => {
        Ok(remote)
      }
      Err(_) => {
        Err(anyhow::anyhow!("No remote branch"))
      }
    }
    
  }

  /// 是否为推送提交
  ///
  pub fn unpushed_commits(&self) -> Result<bool> {
    let repo = &self.repository;
    // 获取远程分支对象
    let remote_branch = self.current_remote_branch_inner();
    if let Err(_) = remote_branch {
      return Ok(false); 
    }
    let remote_branch = remote_branch?;
    let remote_commit_id = remote_branch.get().target().unwrap();
    // 获取本地分支的最新提交
    let local_commit_id = repo.head()?.target().unwrap();
    // 比较本地分支和远程分支的提交历史
    let mut revwalk = repo.revwalk()?;
    revwalk.push(local_commit_id)?;
    revwalk.hide(remote_commit_id)?;
    // 检查是否有未推送的提交
    for _ in revwalk {
      return Ok(true);
    }
    Ok(false)
  }

  /// 获取远程仓库列表
  pub fn remotes(&self) -> Vec<String> {
    let remote_names: Vec<String> = self.branches().unwrap_or(vec![]).iter().filter(|b| {
      b.is_remote
    }).map(|b| {
      b.name.clone()
    }).collect();
    remote_names
  }

  /// 获取仓库的文件状态
  pub fn work_status(&self) -> Result<WorkStatus> {
    let path = self.repository.path().to_str().unwrap();
    let mut statuses: WorkStatus = WorkStatus::None;
    match self.untracked_files() {
      Ok(untracks) => {
        if !untracks.is_empty() {
          statuses |= WorkStatus::Untracked;
        }
      }
      _ => {
        error!("No untracked files found {}", path);
      }
    }
    match self.workspace_change() {
      Ok(workspace_change) => {
        if !workspace_change.is_empty() {
          statuses |= WorkStatus::Modified;
        }
      }
      _ => {
        error!("No modified files found {}", path);
      }
    }
    match self.uncommitted() {
      Ok(uncommited) => {
        if uncommited {
          statuses |= WorkStatus::Uncommitted;
        }
      }
      Err(_) => {}
    }
    match self.unpushed_commits() {
      Ok(unpushed) => {
        if unpushed {
          statuses |= WorkStatus::Unpushed;
        }
      }
      _ => {
        error!("No unpushed commits found {}", path);
      }
    }
    if statuses.is_empty() {
      statuses |= WorkStatus::Ok;
    }
    Ok(statuses)
  }

  pub fn get_branch(&self, branch_name: &str) -> Result<Branch> {
    let branches = self.branches()?;
    for branch in branches {
      if branch.name == branch_name {
        return Ok(branch);
      }
    }
    Err(anyhow::anyhow!("Branch not found"))
  }

  pub fn build_commits(&self, mut revwalk: Revwalk, count: i32) -> Result<Box<Vec<Commit>>> {
    let mut commits: Box<Vec<Commit>> = Box::new(Vec::new());
    for (i, id) in revwalk.by_ref().take(count as usize).enumerate() {
      let id = id?;
      let commit = self.repository.find_commit(id)?;
      let commit = build_commit(&commit, self.workdir());
      commits.push(commit);
    }
    Ok(commits)
  }

  /// 从当前HEAD获取所有之前的提交
  ///
  pub fn commits(&self, count: i32) -> Result<Box<Vec<Commit>>> {
    let head_id = self.repository.head()?.target().unwrap();
    let mut revwalk = self.repository.revwalk()?;
    revwalk.push(head_id)?;
    let commits = self.build_commits(revwalk, count)?;
    Ok(commits)
  }

  /// 根据commit_id 获取之前的指定数量提交
  ///
  pub fn get_commits_before(
    &self,
    commit_id: impl Into<Oid>,
    count: i32,
  ) -> Result<Box<Vec<Commit>>> {
    let commits = self.repository.find_commit(commit_id.into())?;
    let mut revwalk = self.repository.revwalk()?;
    revwalk.push(commits.id())?;
    let commits = self.build_commits(revwalk, count)?;
    Ok(commits)
  }

  /// 获取分支所有提交，gix中的提交对象
  fn branch_commit_inner(&self, branch: &Branch) -> Result<git2::Commit> {
    let branch_reference = self.repository.find_reference(&branch.reference);
    if let Err(_) = branch_reference {
      return Err(anyhow::anyhow!("Branch not found"));
    }
    let branch_reference = branch_reference?;
    let branch_commit = branch_reference.peel_to_commit();
    if let Err(_) = branch_commit {
      return Err(anyhow::anyhow!("Branch not found"));
    }
    Ok(branch_commit?)
  }

  /// 获取分支的提交
  ///
  pub fn get_branch_commits(&self, branch: &Branch, count: i32) -> Result<Box<Vec<Commit>>> {
    // 获取分支所在的提交
    let commit = self.branch_commit_inner(branch)?;
    let mut revwalk = self.repository.revwalk()?;
    revwalk.push(commit.id())?;
    let commits = self.build_commits(revwalk, count)?;
    Ok(commits)
  }

  /// 获取一个提交的内容
  ///
  pub fn commit_content(&self, commit_id: impl Into<Oid>) -> Result<Vec<File>> {
    let repo = &self.repository;
    let mut files: Vec<File> = Vec::new();
    // 获取父提交
    let commit = repo.find_commit(commit_id.into())?;
    let now_tree = commit.tree()?;
    let old_tree = commit.parent(0)?.tree()?;
    // 对比两个树的差异
    let diff = repo.diff_tree_to_tree(Some(&old_tree), Some(&now_tree), None)?;
    let delta = diff.deltas();
    // 遍历差异, 获取差异的文件
    for delta in delta {
        let path = delta.new_file().path().unwrap().to_str().unwrap().to_string();
        let size = delta.new_file().size();
        let status = change_status_to_file_status(&delta.status());
        let new_blob = repo.find_blob(delta.new_file().id());
        let new_id = delta.new_file().id().to_string();
        let old_id = delta.old_file().id().to_string();
        let (exist, _) = match new_blob {
            Ok(blob) => (true, blob.content().to_vec()),
            Err(_) => (false, Vec::new()),
        };
        let file = File::new(
            path,
            size as usize,
            status,
            new_id,
            old_id,
            exist
        );
        files.push(file);
    }
    Ok(files)
  }

  /// 获取分支的贡献者
  pub fn authors(&self, branch: &Branch) -> Result<Vec<Author>> {
    let mut lasted_commit_id = Option::<Oid>::None;
    let mut author_set = HashSet::new();
    // 获取缓存
    let cache_value = self.authors_cache(branch);
    if let Some(cache_value) = cache_value {
      author_set.extend(cache_value.0);
      lasted_commit_id = Some(cache_value.1);
    }
    // 1. 获取分支的提交
    let branch_commit = self.branch_commit_inner(branch)?;
    let mut revwalk = self.repository.revwalk()?;
    revwalk.push(branch_commit.id())?;
    // 2. 获取提交的作者, 获取作者的邮箱
    for commit in revwalk.by_ref().into_iter() {
      let commit = commit?;
      // 如果当前提交的id和缓存的id相同，说明之后的记录都已经缓存过了，直接退出
      if lasted_commit_id.is_some() && commit == lasted_commit_id.unwrap() {
        break;
      }
      let commit_obj = self.repository.find_commit(commit)?;
      let author = commit_obj.author();
      author_set.insert(Author::new(
        author.name().unwrap().to_string(),
        author.email().unwrap().to_string(),
      ));
    }
    let lasted_id = branch_commit.id().to_string();
    let authors: Vec<Author> = author_set.into_iter().collect();
    // 3. 设置缓存
    self.set_authors_cache(authors.clone(), branch, lasted_id.parse()?);
    Ok(authors)
  }

  /// 获取提交者缓存
  fn authors_cache(&self, branch: &Branch) -> Option<(Vec<Author>, Oid)> {
    if let Some(cache) = self.cache.borrow().as_ref() {
      let authors = cache.branch_authors(self.repository.path().to_str().unwrap(), branch);
      if let Some((authors, last_commit_id)) = authors {
        return Some((authors, last_commit_id));
      }
    }
    None
  }
  /// 设置提交者缓存
  ///
  pub fn set_authors_cache(&self, authors: Vec<Author>, branch: &Branch, lasted_id: Oid) {
    if let Some(cache) = self.cache.borrow_mut().as_mut() {
      cache.set_authors(self.repository.path().to_str().unwrap(), &authors, branch, &lasted_id);
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use std::process::Command;

  #[test]
  fn test_untracked_files() {
    let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
    println!(
      "test_untracked_files: {:?}",
      provider.untracked_files().unwrap()
    );
  }

  #[test]
  fn test_has_modified_files() {
    let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
    println!(
      "test_has_modified_files： {:?}",
      provider.workspace_change()
    );
  }

  #[test]
  fn test_modified_files() {
    let provider = GitDataProvider::new(r"E:\workSpace\git\test_repo").unwrap();
    println!(
      "test_modified_files： {:?}",
      provider.workspace_change().unwrap()
    );
  }

  #[test]
  fn test_uncommit_files() {
    let provider = GitDataProvider::new(r"E:\workSpace\git\test_repo").unwrap();
    let output = if cfg!(target_os = "windows") {
      Command::new("cmd")
       .arg("/C")
       .arg("git")
       .arg("status")
       .arg("-s")
       .current_dir(r"E:\workSpace\git\test_repo")
       .output()
      .expect("failed to execute process")
    } else {
      Command::new("sh")
        .arg("-c")
        .arg("git status -s")
        .output()
        .expect("failed to execute process")
    };
    match output.status.success() {
      true => {
        let output = String::from_utf8_lossy(&output.stdout);
        for line in output.lines() {
          let status = &line[0..2];
          let path = &line[3..];
          if status.chars().nth(0).unwrap() != ' ' {
            println!("{}", path);
          }
        }
      } 
      false => {
        println!("{}", String::from_utf8_lossy(&output.stderr)); 
      }
    }
  }


  #[test]
  fn test_unpushed_commits() {
    let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
    println!(
      "test_unpushed_commits: {}",
      provider.unpushed_commits().unwrap()
    );
  }

  #[test]
  fn test_branches() {
    let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
    for branch in provider.branches().unwrap() {
      println!("{:?}", branch)
    }
  }

  #[test]
  fn test_branch_commit() {
    let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
    let branch = Branch::new(
      "gh-pages".to_string(),
      true,
      "refs/remotes/origin/gh-pages".to_string(),
    );
    let commits = provider.get_branch_commits(&branch, 1000).unwrap();
    for commit in commits.iter() {
      println!("{:?}", commit);
    }
  }

  #[test]
  fn valid_provider() {
    let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL");
    match provider {
      Ok(_) => {
        println!("valid_provider");
      }
      Err(_) => {
        println!("invalid_provider");
      }
    }
  }
}
