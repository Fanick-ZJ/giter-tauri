use crate::types::fs;
use crate::types::fs::Dir;
use crate::util::build_commit;
use crate::util::change_status_to_file_status;
use crate::util::get_file_content;
use crate::util::is_binary_file;
use crate::util::is_binary_file_content;
use crate::util::object_is_binary;
use crate::util::size_by_path;
use crate::util::stamp_to_ymd;
use crate::util::str_to_oid;
use crate::util::time_to_ymd;
use crate::util::write_file;
use anyhow::anyhow;
use anyhow::Result;
use git2::build::CheckoutBuilder;
use git2::Cred;
use git2::CredentialType;
use git2::FetchOptions;
use git2::PushOptions;
use git2::RemoteCallbacks;
use git2::Tree;
use git2::TreeWalkMode;
use git2::TreeWalkResult;
use git2::{BranchType, Oid, Repository, Revwalk, Status};
use serde_json::Value;
use similar::DiffOp;
use similar::TextDiff;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fmt::Pointer;
use std::i32;
use std::process::Command;
use std::io::{self, ErrorKind};
use std::path::Path;
use std::path::PathBuf;
use std::usize;
use std::vec;
use std::os::windows::process::CommandExt;
use super::file::FileHistoryEntry;
use super::{author::Author, branch::Branch, commit::Commit, status::WorkStatus};

use super::commit_filter::FilterConditions;
use super::contribution::CommitStatistic;
use super::diff::ContentDiff;
use super::file::ChangedFile;
use super::file::CommittedFile;
use super::file::UntrackedFile;
use super::status::status_to_changed_status;
use super::status::FileStatus;
use super::error::GitUtilsErrorCode;

pub struct GitDataProvider {
    pub repository: Repository,
}

impl Pointer for GitDataProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitDataProvider")
            .field(
                &format!("{:p}", &self.repository).to_string(),
                &"".to_string(),
            )
            .finish()
    }
}

impl GitDataProvider {
    pub fn new<P: AsRef<Path>>(repository: P) -> Result<Self, GitUtilsErrorCode> {
        let repo = Repository::open(&repository);
        match repo {
            Ok(repo) => Ok(GitDataProvider {
                repository: repo,
            }),
            Err(err) => {
                match err.code() {
                    git2::ErrorCode::NotFound => Err(GitUtilsErrorCode::RepoNotFound(repository.as_ref().display().to_string())),
                    git2::ErrorCode::Owner => Err(GitUtilsErrorCode::NoOwner(repository.as_ref().display().to_string())),
                    _ => Err(GitUtilsErrorCode::OtherError(err.message().to_string())),
                }
            }
        }
    }

    pub fn workdir(&self) -> &Path {
        self.repository.workdir().unwrap()
    }

    pub fn author(&self) -> Result<Author, GitUtilsErrorCode> {
        let config = self.repository.config()?;
        let name = config.get_string("user.name")?;
        let email = config.get_string("user.email")?;
        Ok(Author { name, email })
    }

    /// 根据仓库文件的相对地址，获取文件的绝对地址
    /// 
    fn blob_path<T: AsRef<Path>>(&self, path: T) -> PathBuf {
        let path = path.as_ref();
        let path = self.workdir().join(path);
        path
    }

    /// 获取位追踪的文件列表
    ///
    pub fn untracked_files(&self) -> Result<Vec<UntrackedFile>, GitUtilsErrorCode> {
        let status = self.repository.statuses(None)?;
        let mut untracks: Vec<UntrackedFile> = Vec::new();
        for item in &status {
            if item.status() == Status::WT_NEW {
                let abs_path = self.blob_path(item.path().unwrap());
                let size = size_by_path(&abs_path)? as usize;
                let is_binary = is_binary_file(&abs_path)
                    .map_err(|e| GitUtilsErrorCode::OtherError(e.to_string()))?;
                let untracked_file = UntrackedFile::new(&abs_path, size, is_binary);
                untracks.push(untracked_file);
            }
        }
        Ok(untracks)
    }

    ///工作空间是否有修改
    ///
    pub fn workspace_change(&self) -> Result<Vec<String>, GitUtilsErrorCode> {
        let status = self.repository.statuses(None)?;
        let mut modified: Vec<String> = Vec::new();
        let index_status = Status::WT_DELETED.bits()
            | Status::WT_MODIFIED.bits()
            | Status::WT_NEW.bits()
            | Status::WT_RENAMED.bits()
            | Status::WT_TYPECHANGE.bits();
        for item in &status {
            let bits = item.status().bits();
            if bits & index_status > 0 {
                modified.push(item.path().unwrap_or("").to_string());
            }
        }
        Ok(modified)
    }

    pub fn uncommitted(&self) -> Result<bool, GitUtilsErrorCode> {
        let status = self.repository.statuses(None)?;
        for item in &status {
            let bits = item.status().bits();
            let index_status = Status::INDEX_DELETED.bits()
                | Status::INDEX_MODIFIED.bits()
                | Status::INDEX_NEW.bits()
                | Status::INDEX_RENAMED.bits()
                | Status::INDEX_TYPECHANGE.bits();
            if (bits & index_status) > 0 {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// 根据路径获取文件的 Oid， 先从当前的树中查找，再从索引中查找，没有的话，再去index中查找
    pub fn get_path_oid(&self, path: &PathBuf) -> Result<Oid> {
        // 获取 HEAD 引用
        let reference = self.repository.head()?;
        // 剥离引用为 Commit
        let commit = reference.peel_to_commit()?;
        // 获取 Commit 的树对象
        let tree = commit.tree()?;
        // 从树对象中查找路径对应的条目
        let entry = tree.get_path(&path);
        if let Err(_) = entry {
           let index = self.repository.index()?; 
           for item in index.iter() {
                if String::from_utf8(item.path.to_vec())? == path.to_str().unwrap().to_string() {
                    return Ok(item.id);
                }
           }
        }
        Ok(entry?.id())
    }

    fn blob_is_binary(&self, oid: Oid) -> Result<bool, GitUtilsErrorCode> {
        let blob = self.repository
            .find_blob(oid)
            .map_err(|_| GitUtilsErrorCode::BlobNotFound(oid.to_string()))?;
        let is = blob.is_binary();
        Ok(is)

    }

    pub fn staged_files(&self) -> Result<Vec<ChangedFile>, GitUtilsErrorCode> {
        let status = self.repository.statuses(None)?;
        let mut modified: Vec<ChangedFile> = Vec::new();
        for item in &status {
            let bits = item.status().bits();
            let index_status = Status::INDEX_DELETED.bits()
                | Status::INDEX_MODIFIED.bits()
                | Status::INDEX_NEW.bits()
                | Status::INDEX_RENAMED.bits()
                | Status::INDEX_TYPECHANGE.bits();
            if (bits & index_status) > 0 {
                let path = PathBuf::from(item.path().unwrap());
                let oid = self.get_path_oid(&path).unwrap_or(Oid::from_str("0").unwrap());
                let status = status_to_changed_status(item.status());
                let changed_file = ChangedFile::new(path, oid, status);
                modified.push(changed_file); 
            } 
        } 
        Ok(modified)
    }

    pub fn changed_files(&self) -> Result<Vec<ChangedFile>, GitUtilsErrorCode> {
        let status = self.repository.statuses(None)?;
        let mut modified: Vec<ChangedFile> = Vec::new(); 
        for item in &status {
            let bits = item.status().bits();
            let index_status = Status::WT_DELETED.bits()
            | Status::WT_MODIFIED.bits()
            | Status::CONFLICTED.bits()
            | Status::WT_NEW.bits()
            | Status::WT_RENAMED.bits()
            | Status::WT_TYPECHANGE.bits()
            | Status::WT_NEW.bits();
            if (bits & index_status) > 0 {
                let path = PathBuf::from(item.path().unwrap());
                let oid = self.get_path_oid(&path).unwrap_or(Oid::from_str("0").unwrap());
                let status = status_to_changed_status(item.status());
                let changed_file = ChangedFile::new(path, oid, status);
                modified.push(changed_file);
            }
        }
        Ok(modified)
    }

    pub fn relative_path(&self, path: &PathBuf) -> Result<PathBuf> {
        let workdir = self.workdir();
        let relative_path = path.strip_prefix(workdir)?;
        Ok(relative_path.to_path_buf()) 
    }

    pub fn add_to_stage(&self, path: &PathBuf) -> Result<(), GitUtilsErrorCode> {
        let repo = &self.repository;
        let mut index = repo.index()?;
        index.add_path(&path)?;
        index.write()?;
        Ok(()) 
    }

    pub fn remove_from_stage(&self, path: &PathBuf) -> Result<(), GitUtilsErrorCode> {
        let repo = &self.repository;
        let staged_files = self.staged_files()?;
        let file = staged_files
            .iter()
            .find(|f| f.path.to_str() == path.to_str())
            .ok_or_else(|| anyhow::anyhow!("File not found in stage"))?;
        let mut index = repo.index()?;
        match file.status {
            // 如果是删除的，就直接删除
            FileStatus::Added => index.remove_path(&path)?,
            _ => {
                let head = repo.head()?.peel_to_commit()?;
                let tree = head.tree()?;
                let entry = tree.get_path(&path)?;
                let blob = repo.find_blob(entry.id())?;
                // 如果能在索引库中找到文件，就把HEAD库中的文件添加到索引库中，以实现移除索引的效果
                let entry = index.iter().find(|e| String::from_utf8(e.path.to_vec()).unwrap() == path.to_str().unwrap().to_string());
                if let Some(entry) = entry {
                    index.add_frombuffer(&entry, blob.content())?;
                } else {
                    return Err(GitUtilsErrorCode::OtherError("File not found in index".to_string()));
                }
            } 
        }
        index.write()?;
        Ok(())
    }

    /// 暂存区文件恢复到工作区
    pub fn checkout_file (&self, path: &PathBuf) -> Result<(), GitUtilsErrorCode> {
        let repo = &self.repository;
        // 判断当前的文件是不是新加的
        let status = self.changed_files()?;
        let file = status
            .iter()
            .find(|f| f.path.to_str() == path.to_str())
            .ok_or_else(|| anyhow::anyhow!("File not found in stage"))?;
        // 如果是新加的，就直接忽略
        if matches!(file.status, FileStatus::Added) {
            return Ok(());
        }
        let mut checkout_opts = CheckoutBuilder::new();
        checkout_opts.path(path).force();
        repo.checkout_head(Some(&mut checkout_opts))?;
        Ok(())
    }

    pub fn current_branch(&self) -> Result<Branch, GitUtilsErrorCode> {
        let repo = &self.repository;
        let branches = repo.branches(None).unwrap();
        for branch in branches {
            let (branch, _) = branch.unwrap();
            if branch.is_head() {
                return Ok(Branch::from(branch.into_reference()));
            }
        }
        Err(GitUtilsErrorCode::CurrentBranchNotFound("No current branch found".to_string()))
    }

    /// 获取分支列表
    pub fn branches(&self) -> Result<Vec<Branch>, GitUtilsErrorCode> {
        let repo = &self.repository;
        let branches = repo.branches(None).unwrap();
        let mut _branches: Vec<Branch> = Vec::new();
        for branch in branches {
            let (branch, _) = branch.unwrap();
            _branches.push(Branch::from(branch.into_reference()))
        }
        Ok(_branches)
    }

    /// 获取当前分支对应的远程分支
    pub fn current_remote_branch(&self) -> Result<Branch, GitUtilsErrorCode>{
        let current = self.current_branch()?;
        let remote = self
            .repository
            .find_branch(&current.reference, BranchType::Local)?
            .upstream();
        match remote {
            Ok(remote) => {
                Ok(Branch::from(remote.into_reference()))
            }
            Err(_) => Err(GitUtilsErrorCode::RemoteNotFound(current.name)),
        }
    }

    fn current_remote_branch_inner(&self) -> Result<git2::Branch, GitUtilsErrorCode> {
        let current = self.current_branch()?;
        let remote = self
            .repository
            .find_branch(&current.reference, BranchType::Local)?
            .upstream();
        match remote {
            Ok(remote) => Ok(remote),
            Err(_) => Err(GitUtilsErrorCode::RemoteNotFound(current.name)),
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
        let remote_names: Vec<String> = self
            .branches()
            .unwrap_or(vec![])
            .iter()
            .filter(|b| b.is_remote)
            .map(|b| b.name.clone())
            .collect();
        remote_names
    }

    /// 获取仓库的文件状态
    pub fn work_status(&self) -> Result<WorkStatus, GitUtilsErrorCode> {
        let mut statuses = WorkStatus::None;

        // 统一错误处理，使用 map_err 转换错误类型
        let untracked = self.untracked_files()
            .map(|v| !v.is_empty())
            .map_err(|e| anyhow::anyhow!(e).context("Failed to get untracked files"))?;

        let modified = self.workspace_change()
            .map(|v| !v.is_empty())
            .map_err(|e| anyhow::anyhow!(e).context("Failed to get workspace changes"))?;

        let uncommitted = self.uncommitted()
            .map_err(|e| anyhow::anyhow!(e).context("Failed to check uncommitted changes"))?;

        let unpushed = self.unpushed_commits()
            .map_err(|e| anyhow::anyhow!(e).context("Failed to check unpushed commits"))?;
        // 使用组合的 Result 处理
        let results = vec![
            (untracked, WorkStatus::Untracked), 
            (modified, WorkStatus::Modified), 
            (uncommitted, WorkStatus::Uncommitted), 
            (unpushed, WorkStatus::Unpushed)
        ];
        for (result, status) in results {
            match result {
                true => statuses |= status,
                false => {}
            }
        }
        // 简化最终状态判断
        if statuses.is_empty() {
            statuses |= WorkStatus::Ok;
        }

        Ok(statuses)
    }

    pub fn get_branch(&self, branch_name: &str) -> Result<Branch, GitUtilsErrorCode> {
        let branches = self.branches()?;
        for branch in branches {
            if branch.name == branch_name {
                return Ok(branch);
            }
        }
        Err(GitUtilsErrorCode::BranchNotFound(branch_name.to_string()))
    }

    pub fn build_commits(&self, revwalk: &mut Revwalk, count: i32) -> Result<Vec<Commit>> {
        let mut commits: Vec<Commit> = Vec::new();
        for (_, id) in revwalk.by_ref().take(count as usize).enumerate() {
            let id = id?;
            let commit = self.repository.find_commit(id)?;
            let commit = build_commit(&commit, &self.repository);
            commits.push(commit);
        }
        Ok(commits)
    }

    /// 从当前HEAD获取所有之前的提交
    ///
    pub fn commits(&self, count: i32) -> Result<Vec<Commit>> {
        let head_id = self.repository.head()?.target().unwrap();
        let mut revwalk = self.repository.revwalk()?;
        revwalk.push(head_id)?;
        let commits = self.build_commits(&mut revwalk, count)?;
        Ok(commits)
    }

    /// 根据commit_id 获取之前的指定数量提交
    ///
    pub fn get_commits_before(
        &self,
        commit_id: impl Into<Oid>,
        count: i32,
    ) -> Result<Vec<Commit>> {
        let commits = self.repository.find_commit(commit_id.into())?;
        let mut revwalk = self.repository.revwalk()?;
        revwalk.push(commits.id())?;
        let commits = self.build_commits(&mut revwalk, count)?;
        Ok(commits)
    }

    /// 获取分支所有提交(分支的最后一次提交)，git2中的提交对象
    fn branch_commit_inner(&self, branch: &Branch) -> Result<git2::Commit, GitUtilsErrorCode> {
        let b_type = if branch.is_remote {
            BranchType::Remote
        } else {
            BranchType::Local
        };
        let branch_reference = self.repository.find_branch(&branch.name, b_type);
        if let Err(_) = branch_reference {
            return Err(GitUtilsErrorCode::BranchNotFound(branch.name.clone()));
        }
        let branch_reference = branch_reference?;
        let branch_commit = branch_reference.get().peel_to_commit();
        if let Err(_) = branch_commit {
            return Err(GitUtilsErrorCode::BranchNotFound(branch.name.clone()));
        }
        Ok(branch_commit?)
    }

    /// 获取分支的提交，返回提交对象和此分支的提交总数
    ///
    pub fn branch_commits(&self, branch: &Branch, count: i32) -> Result<Vec<Commit>, GitUtilsErrorCode> {
        // 获取分支所在的提交
        let commit = self.branch_commit_inner(branch)?;
        let mut revwalk = self.repository.revwalk()?;
        revwalk.push(commit.id())?;
        let commits = self.build_commits(&mut revwalk, count)?;
        Ok(commits)
    }

    /// 在某些情况下，使用git命令获取提交数量会更快，因为git命令是直接调用底层的git库，而不是通过rust的git2库
    pub fn before_reference_commits_count(&self, reference: &str) -> Result<i32, GitUtilsErrorCode> {
        let output = Command::new("git")
            .args(&["rev-list", "--count", reference])
            .current_dir(self.workdir())
            .output()?;

        if output.status.success() {
            let commit_count = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<usize>()
                .map_err(|_| io::Error::new(ErrorKind::InvalidData, "无法解析提交数量"))?;

            Ok(commit_count as i32)
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            return Err(GitUtilsErrorCode::OtherError(error_message.to_string()));
        }
    }

    /// 获取一颗提交树的所有文件列表
    fn tree_walk(&self, tree: &git2::Tree) -> Vec<CommittedFile> {
        let mut files: Vec<CommittedFile> = Vec::new();
        let _ = tree.walk(TreeWalkMode::PostOrder, |root, entry| {
            let path = Path::new(root);
            let path = path.join(entry.name().unwrap()).to_str().unwrap().to_string();
            let blob = self.repository.find_blob(entry.id());
            if blob.is_err() {
                files.push(CommittedFile::new(path, 0, FileStatus::Added, entry.id().to_string(), "0".repeat(20), false, false, false));
            } else {
                let blob = blob.unwrap();
                let size = blob.size();
                let status = FileStatus::Added;
                let object_id = entry.id().to_string();
                let is_binary = blob.is_binary();
                files.push(CommittedFile::new(path, size, status, object_id, "0".repeat(20), true, is_binary, false));
            }
            1
        });
        files
    }

    pub fn get_commit (&self, commit_id: impl Into<Oid>) -> Result<Commit, GitUtilsErrorCode> {
        let commit = self.repository.find_commit(commit_id.into())?;
        let commit = build_commit(&commit, &self.repository);
        Ok(commit)
    }

    /// 获取一个提交的内容
    ///
    pub fn commit_content(&self, commit_id: impl Into<Oid>) -> Result<Vec<CommittedFile>, GitUtilsErrorCode> {
        let repo = &self.repository;
        // 获取父提交
        let commit = repo.find_commit(commit_id.into())?;
        let now_tree = commit.tree()?;
        let old_tree = commit.parents()
            .next()
            .and_then(|parent| parent.tree().ok());
        // 当没找到父提交时，说明是第一个提交，单独处理
        if old_tree.is_none() {
            let files = self.tree_walk(&now_tree);
            return Ok(files);
        }
        let mut files: Vec<CommittedFile> = Vec::new();
        // 对比两个树的差异
        let diff = repo.diff_tree_to_tree(old_tree.as_ref(), Some(&now_tree), None)?;
        let deltas = diff.deltas();
        // 遍历差异, 获取差异的文件
        for delta in deltas {
            let (old_file, new_file) = (delta.old_file(), delta.new_file());
            let path = new_file.path()
                .ok_or_else(|| anyhow::anyhow!("new file path is not valid utf-8"))?
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("new file path is not valid utf-8"))?
                .to_string();
            // git2的is_blob函数好像有问题，直接用文件内容判断吧
            let (is_binary, old_is_binary) = (object_is_binary(new_file.id(), repo), object_is_binary(old_file.id(), repo));
            let size = new_file.size();
            let status = change_status_to_file_status(&delta.status());
            let new_blob = repo.find_blob(delta.new_file().id());
            let new_id = new_file.id().to_string();
            let old_id = old_file.id().to_string();
            let exist = match new_blob {
                Ok(_) => true,
                Err(_) => false,
            };
            let file = CommittedFile::new(path, size as usize, status, new_id, old_id, exist, is_binary, old_is_binary);
            files.push(file);
        }
        Ok(files)
    }

    /// 根据oid获取文件内容
    /// oid: 提交id
    pub fn get_blob_content(&self, oid: impl Into<Oid>) -> Result<Vec<u8>, GitUtilsErrorCode> {
        let oid = oid.into();
        let blob = self.repository.find_blob(oid);
        if blob.is_err() {
            return Err(GitUtilsErrorCode::BlobNotFound(oid.to_string()));
        }
        let blob = blob.unwrap();
        Ok(blob.content().to_vec())
    }

    /// 获取文件的差异
    /// old: 旧的文件内容
    /// new: 新的文件内容
    ///
    pub fn get_file_diff(&self, old: impl Into<Oid>, new: impl Into<Oid>) -> Result<(Vec<DiffOp>, String), GitUtilsErrorCode> {
        let old = old.into();
        let new = new.into();
        let old_blob = self.repository.find_blob(old);
        let new_blob = self.repository.find_blob(new);
        if old_blob.is_err() {
            return Err(GitUtilsErrorCode::BlobNotFound(old.to_string()));
        }
        if new_blob.is_err() {
            return Err(GitUtilsErrorCode::BlobNotFound(new.to_string()));
        }
        let old_blob = old_blob.unwrap();
        let new_blob = new_blob.unwrap();
        let old_content = String::from_utf8(old_blob.content().to_vec())?;
        let new_content = String::from_utf8(new_blob.content().to_vec())?;
        let diff = TextDiff::from_lines(&old_content, &new_content);
        let diff_display = diff.unified_diff().missing_newline_hint(false).to_string();
        Ok((diff.ops().iter().map(|op| op.clone()).collect(), diff_display))
    }

    /// 获取文件内容、差异
    /// 
    pub fn get_file_content_diff(&self, old: impl Into<Oid>, new: impl Into<Oid>) -> Result<ContentDiff, GitUtilsErrorCode> {
        let old = old.into();
        let new = new.into();
        let old_blob = self.get_blob_content(old)?;
        let new_blob = self.get_blob_content(new)?;
        let diff = self.get_file_diff(old, new)?;
        Ok(ContentDiff {
            old: String::from_utf8(old_blob)?,
            new: String::from_utf8(new_blob)?,
            ops: diff.0,
            display: diff.1
        })
    }

    /// 获取分支的贡献者
    pub fn authors(&self, branch: &Branch) -> Result<Vec<Author>, GitUtilsErrorCode> {
        let mut lasted_commit_id = Option::<Oid>::None;
        let mut author_set = HashSet::new();
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
            let author = self.get_commit_author(&commit)?;
            author_set.insert(author);
        }
        let lasted_id = branch_commit.id();
        let authors: Vec<Author> = author_set.into_iter().collect();
        Ok(authors)
    }

    pub fn get_commit_author(&self, commit_id: &Oid) -> Result<Author, GitUtilsErrorCode> {
        let commit = self.repository.find_commit(commit_id.clone())?;
        let author = commit.author();
        // 使用 from_utf8_lossy 处理非 UTF-8 编码
        let name_bytes = author.name_bytes();
        let email_bytes = author.email_bytes();
        let name = String::from_utf8_lossy(name_bytes).into_owned();
        let email = String::from_utf8_lossy(email_bytes).into_owned();
        
        Ok(Author::new(name, email))
            
    }

    
    /// 获取分支提交贡献统计
    pub fn get_branch_commit_contribution(&self, branch: &Branch) -> Result<Vec<CommitStatistic>> {
        let commits = self.branch_commit_inner(branch)?;
        let mut revwalk = self.repository.revwalk()?;
        revwalk.push(commits.id())?;
        let mut map = HashMap::<String, CommitStatistic>::new();
        for commit in revwalk.by_ref().into_iter() {
            if let Ok(commit) = commit {
                let author = self.get_commit_author(&commit)?;
                let commit = self.repository.find_commit(commit)?;
                let time = stamp_to_ymd(commit.time().seconds());
                if let Err(_) = time{
                    continue;
                }
                let time = time.unwrap();
                if !map.contains_key(author.email.as_str()) {
                    map.insert(author.email.clone(), CommitStatistic::new(self.workdir().to_path_buf(), branch.clone(), author.clone()));      
                }
                let stat = map.get_mut(author.email.as_str()).unwrap();
                let _ = stat.add(time, 1);
            } else {
                continue;
            }
        }
        Ok(map.into_values().collect())
    }

    /// 获取根据筛选条件过滤后的命令行
    pub fn build_cinnut_filter(&self, reference: &str, filter: &HashMap<String, Value>) -> Result<Command, GitUtilsErrorCode> {
        let filter = FilterConditions::build_from_sv_map(filter);
        let mut cmd = Command::new("git");
        cmd.creation_flags(0x08000000);
        cmd.current_dir(self.workdir());
        cmd.args(&["rev-list", reference]);
        if let Some(author) = filter.author {
            cmd.args(&["--author", &format!("{}", author.name)]); 
        }
        if let Some(start_time) = filter.start_time {
            let formated = time_to_ymd(start_time)?;
            cmd.args(&["--since", & formated]); 
        }
        if let Some(end_time) = filter.end_time {
            let formated = time_to_ymd(end_time)?;
            cmd.args(&["--until", & formated]); 
        }
        if let Some(offset) = filter.offset {
            cmd.args(&["--skip", &offset.to_string()]);
        }
        if let Some(count) = filter.count {
            cmd.args(&["--max-count", &count.to_string()]); 
        }
        if let Some(message) = filter.message {
            cmd.args(&["--grep", &message]);
        }
        Ok(cmd)
    }

    pub fn reference_commit_filter_count(&self, reference: &str, filter: &HashMap<String, Value>, offset: Option<i32>, size: Option<i32>) -> Result<i32, GitUtilsErrorCode> {
        let mut filter = filter.clone();
        filter.entry("offset".to_string()).or_insert(Value::from(offset.unwrap_or(0)));
        filter.entry("count".to_string()).or_insert(Value::from(size.unwrap_or(i32::MAX)));
        let mut cmd = self.build_cinnut_filter(reference, &filter)?;
        cmd.args(&["--count"]);
        let output = cmd.output()?;
        let count = String::from_utf8_lossy(&output.stdout)
           .trim()
           .parse::<usize>().map_err(|e| anyhow!(e.to_string()))?; 
        Ok(count as i32)
    }

    pub fn reference_commit_filter_details(&self, reference: &str, filter: &HashMap<String, Value>, offset: Option<i32>, size: Option<i32>) -> Result<Vec<Commit>, GitUtilsErrorCode> {
        let mut filter = filter.clone();
        filter.entry("offset".to_string()).or_insert(Value::from(offset.unwrap_or(0)));
        filter.entry("count".to_string()).or_insert(Value::from(size.unwrap_or(i32::MAX)));
        let mut cmd = self.build_cinnut_filter(reference, &filter)?;
        let output = cmd.output()?;
        let commit_ids = String::from_utf8_lossy(&output.stdout)
          .trim()
         .split("\n")
         .map(|s| s.to_string())
         .collect::<Vec<String>>();
        if commit_ids.len() == 1 && commit_ids[0].is_empty() {
            return Ok(vec![]);
        }
        let mut commits = Vec::<Commit>::new();
        for commit_id in commit_ids {
           let commit = Commit::from_oid(str_to_oid(&commit_id)?, &self.repository)?; 
           commits.push(commit);
        }
        Ok(commits)

    }

 
    pub fn commit (&self, message: &str, update_ref: Option<&str>) -> Result<Oid, GitUtilsErrorCode> {
        // 如果没有指定更新的分支，默认更新当前分支
        let update_ref =  if update_ref.is_none() {
           Some("HEAD")
        } else  {
            update_ref
        };
        let repo = &self.repository;
        if repo.is_bare() {
            return Err(GitUtilsErrorCode::RepoIsBare(repo.path().display().to_string()));
        }
        let mut index = repo.index()?;
        let conflicts = index.conflicts()?;
        if conflicts.into_iter().count() > 0 {
            return Err(GitUtilsErrorCode::RepoHasConflicts(repo.path().display().to_string())); 
        }
        let author = repo.signature();
        if author.is_err() {
            return Err(GitUtilsErrorCode::RepoAuthorNoConfig);
        }
        if self.staged_files()?.len() == 0 {
            return Err(GitUtilsErrorCode::NoStagedFile);
        }
        let author = author.unwrap();
        let tree_oid = index.write_tree()?;
        let tree = repo.find_tree(tree_oid)?;
        let parents = &[&repo.head()?.peel_to_commit()?];
        // 创建提交
        let oid = repo.commit(update_ref, &author, &author, message, &tree, parents)?;
        return Ok(oid);
    }

    pub fn has_tracking(&self, branch: &git2::Branch) -> bool {
        let tracking = branch.upstream();
        match tracking {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn build_remote_credentials_cb(&self, credentials: &Option<(String, String)>) -> impl Fn(&str, Option<&str>, CredentialType) -> Result<Cred, git2::Error> + '_ {
        let (username, password) = credentials.as_ref()
            .map(|(u, p)| (u.clone(), p.clone()))
            .unwrap_or_default();
        move |url: &str, username_from_url: Option<&str>, allowed_types: CredentialType| {
            if let CredentialType::USER_PASS_PLAINTEXT = allowed_types {
                if username.is_empty() || password.is_empty() {
                    return Err(git2::Error::new(
                        git2::ErrorCode::User,
                        git2::ErrorClass::None,
                        "Credentials required for remote operation",
                    ));
                }
                return Cred::userpass_plaintext(&username, &password);
            }
    
            match allowed_types {
                CredentialType::SSH_KEY => 
                    Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")),
                _ => Cred::default()
            }
        }
    }


    pub fn push(&self, remote: &str, branch_name: &str, credentials: Option<(String, String)>) -> Result<(), GitUtilsErrorCode> {
        let repo = &self.repository;
        
        // 提取远程获取和分支验证逻辑
        let mut remote = repo.find_remote(remote)
            .map_err(|_| GitUtilsErrorCode::RemoteNotFound(branch_name.to_string()))?;

        let branch = repo.find_branch(branch_name, BranchType::Local)
            .map_err(|_| GitUtilsErrorCode::BranchNotFound(branch_name.to_string()))?;
        
        // 验证分支跟踪状态
        if !self.has_tracking(&branch) {
            return Err(GitUtilsErrorCode::BranchNotTrackAny(branch_name.to_string()));
        }

        // 提取公共回调配置
        let build_callbacks = || {
            let mut cbs = RemoteCallbacks::new();
            cbs.credentials(self.build_remote_credentials_cb(&credentials));
            cbs
        };

        // 统一错误处理函数
        let handle_error = |e: git2::Error| {
            log::error!("Git operation error: {:?}", e);
            match e.code() {
                git2::ErrorCode::User => GitUtilsErrorCode::PushNeedNameAndPassword,
                _ => return GitUtilsErrorCode::PushOtherError.into()
            }
        };

        // 获取必要分支信息
        let remote_branch = branch.upstream()?;
        let branch_ref = branch.into_reference();
        let branch_ref_name = branch_ref.name().ok_or(anyhow!(""))?;

        // 执行fetch操作
        let mut fetch_opt = FetchOptions::new();
        fetch_opt.remote_callbacks(build_callbacks());
        remote.fetch(&[branch_ref_name], Some(&mut fetch_opt), None)
            .map_err(handle_error)?;

        // 验证祖先关系
        let remote_commit = remote_branch.into_reference().peel_to_commit()?;
        let remote_head = remote_commit.id();
        let local_commit = branch_ref.peel_to_commit()?;
        if !local_commit.parent_ids().any(|id| id == remote_head) {
            return Err(GitUtilsErrorCode::RemoteHeadHasNotInLocal.into());
        }

        // 执行push操作
        let mut push_opt = PushOptions::new();
        push_opt.remote_callbacks(build_callbacks());
        remote.push(&[branch_ref_name], Some(&mut push_opt))
            .map_err(handle_error)?;
        Ok(())
    }

    pub fn pull(&self, remote: &str, branch: &str, credentials: Option<(String, String)>) -> Result<(), GitUtilsErrorCode> {
        let repo = &self.repository;
    
        // 1. 获取远程引用
        let mut remote = repo.find_remote(remote)
            .map_err(|e| {
                log::error!("Find remote error: {:?}", e);
                GitUtilsErrorCode::RemoteNotFound(remote.to_string())
            })?;
        // 2. 配置回调（复用已有的凭证处理逻辑）
        let callbacks = self.build_remote_credentials_cb(&credentials);
        // 3. 执行 fetch 操作
        let mut fetch_opts = FetchOptions::new();
        fetch_opts.remote_callbacks({
            let mut cb = RemoteCallbacks::new();
            cb.credentials(callbacks);
            cb
        });
        // 获取本地分支
        let local_branch = repo.find_branch(branch, BranchType::Local)
            .map_err(|_| GitUtilsErrorCode::BranchNotFound(branch.to_string()))?;
        let local_branch_ref = local_branch.into_reference();

        // 4. 获取FETCH_HEAD 提交
        remote.fetch(&[local_branch_ref.name().unwrap()], Some(&mut fetch_opts), None)
           .map_err(|e| {
            log::error!("Fetch error: {:?}", e);
            GitUtilsErrorCode::PushOtherError
        })?;
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        // 5. 执行合并操作
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_up_to_date() {
            return Ok(());
        }
        else if analysis.0.is_fast_forward() {
            let mut reference = local_branch_ref;
            /*
             * set_target 核心功能：
             * 原子性更新引用：将本地分支引用指向 fetch_commit.id() 对应的提交
             * 实现快进合并：当远程分支是本地分支的直接祖先时，直接移动分支指针
             * 写入引用日志：第二个参数 "Fast-Forward" 会写入 .git/logs 中的引用日志
             */
            let _ = reference.set_target(fetch_commit.id(), "Fast-Forward").map_err(|e| {
                log::error!("Set target error: {:?}", e);
                GitUtilsErrorCode::TargetReferenceNotDirect
            })?;
            repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
            return Ok(());
        }
        else if analysis.0.is_normal() {
            // 创建提交的合并
            let head_commit = repo.head()?.peel_to_commit()?;
            repo.merge(&[&fetch_commit], None, None).map_err(|e| {
                log::error!("Merge error: {:?}", e);
                GitUtilsErrorCode::CommitBeforePullWouldBeOverwrittenByMerge
            })?;
            // 创建合并提交
            let signature = repo.signature().map_err(|_| GitUtilsErrorCode::UserUnConfigured)?;
            let tree_id = repo.index()?.write_tree()?;
            let tree = repo.find_tree(tree_id)?;
            let _ = repo.commit(
                Some("HEAD"),
                &signature, 
                &signature, 
                "Merge commit", 
                &tree, 
                &[&head_commit, &fetch_head.peel_to_commit()?])
                .map_err(|e| {
                    log::error!("Commit error: {:?}", e);
                    GitUtilsErrorCode::BuildMergeCommitError
                })?;
        }
        else if analysis.0.is_none() {
            return Err(GitUtilsErrorCode::CantPull.into());
        }
        Ok(())
    }

    /// 检查两个分支是否存在合并冲突
    pub fn check_branch_conflict(&self, branch_a: impl Into<Branch>, branch_b: impl Into<Branch>) -> Result<bool, GitUtilsErrorCode> {
        let repo = &self.repository;
        let branch_a = branch_a.into();
        let branch_b = branch_b.into();
        // 获取两个分支的最新提交
        let commit_a = self.branch_commit_inner(&branch_a)?;
        let commit_b = self.branch_commit_inner(&branch_b)?;

        // 创建合并基线（共同祖先）
        let ancestor_oid = repo.merge_base(commit_a.id(), commit_b.id())?;
        let ancestor = repo.find_commit(ancestor_oid)?;

        // 获取三个树对象（祖先、分支A、分支B）
        let ancestor_tree = ancestor.tree()?;
        let tree_a = commit_a.tree()?;
        let tree_b = commit_b.tree()?;

        // 执行三方合并
        let mut merge_opts = git2::MergeOptions::new();
        merge_opts.fail_on_conflict(false); // 允许继续检测冲突
        let merged_index = repo.merge_trees(&ancestor_tree, &tree_a, &tree_b, Some(&mut merge_opts))?;

        // 检查冲突文件数量
        Ok(merged_index.has_conflicts())
    }
    
    /// 切换分支
    pub fn switch_branch(&self, branch: &Branch) -> Result<(), GitUtilsErrorCode> {
        let staged = self.staged_files()?;
        if staged.len() > 0 {
            return Err(GitUtilsErrorCode::OtherError("There are staged files, please commit them first".to_string())); 
        }
        let repo = &self.repository;
        let branch_name = branch.name.to_string();

        // 获取目标分支的树对象
        let target_commit = self.branch_commit_inner(branch)?;
        let target_tree = target_commit.tree()?;
        let branch = repo.find_branch(&branch_name, BranchType::Local)
           .map_err(|_| GitUtilsErrorCode::BranchNotFound(branch_name.clone()))?;
        let branch_ref = branch.into_reference();
        let branch_ref_name = branch_ref.name().ok_or(anyhow!(""))?;

        // 获取工作区的修改文件
        let work_changed = self.workspace_change()?;

        // 比较工作区与目标分支的差异
        let diff = repo.diff_tree_to_workdir_with_index(Some(&target_tree), None)?;
        let deltas = diff.deltas();
        
        // 收集冲突文件
        let mut conflicts = Vec::new();
        for delta in deltas {
            let new_file = delta.new_file();
            
            if let Some(path) = new_file.path() {
                let path = &path.to_string_lossy().into_owned();
                // 如果冲突的文件不是工作区的修改文件，跳过
                if !work_changed.contains(path) {
                    continue; 
                }
                conflicts.push(path.to_string());
            }
        }
        if conflicts.len() > 0 {
            return Err(GitUtilsErrorCode::SwitchWillBeOverwrittenByMerge(conflicts.join("\n"))); 
        }
        let mut blob_map = HashMap::new();
        // 获取修改的文件内容，在切换之后再恢复
        for file in work_changed {
           let abs_path = self.blob_path(file); 
           let blob = get_file_content(&abs_path)?;
           blob_map.insert(abs_path, blob);
        }
        repo.set_head(branch_ref_name)
          .map_err(|e| GitUtilsErrorCode::Git2Error(e))?;
        let _ = repo.checkout_head(Some(CheckoutBuilder::default().force()));
        // 恢复修改的文件内容
        for (path, blob) in blob_map {
            let _ = write_file(&path, &blob);
        }
        Ok(())
    }

    /// 根据文件的oid获取文件的历史Oid和所在提交的oid
    pub fn file_history(&self, file_path: String) -> Result<Vec<FileHistoryEntry>, GitUtilsErrorCode> {
        let repo = &self.repository;
        let mut cmd = Command::new("git");
        cmd.creation_flags(0x08000000);
        cmd.current_dir(self.workdir());
        cmd.args(&["log", "--follow","--format=%H", "--", &file_path]);

        let output = cmd.output()?;
        if!output.status.success() {
            return Err(GitUtilsErrorCode::OtherError(format!("git log failed: {}", String::from_utf8_lossy(&output.stderr))));
        }
        /*
         * collect 的魔法： collect::<Result<Vec<_>, _>>() 的特殊性在于，
         * 它可以将 Iterator<Item = Result<T, E>> 转换为 Result<Vec<T>, E>。这个操作会：
         * 如果所有元素都是 Ok(T)，则合并为 Ok(Vec<T>)
         * 如果遇到任何一个 Err(E)，则立即返回第一个遇到的错误
         */
        let commit_ids = String::from_utf8_lossy(&output.stdout)
          .trim()
          .split("\n")
          .map(|s| str_to_oid(s))
          .collect::<Result<Vec<_>, _>>()?;
        println!("commit_ids: {:?}", commit_ids.len());
        let mut history = Vec::new();
        for commit_oid in commit_ids {
            println!("commit_oid: {:?}", commit_oid);
            let content = self.commit_content(commit_oid)?; 
            for file in content.iter() {
                if file.path == file_path {
                    history.push(FileHistoryEntry::new(Commit::from_oid(commit_oid, repo)?, file.clone())); 
                }
            }
        }
        Ok(history)
    }

    fn resolve_tree(
        &self,
        object_id: Oid,
        tree_path: Option<String>,
    ) -> Result<(Tree<'_>, String), GitUtilsErrorCode> {
        let object = self.repository.find_object(object_id, None)?;
        let kind = object.kind().ok_or(GitUtilsErrorCode::OtherError(format!(
            "Unknown object kind: {}",
            object_id
        )))?;

        let (tree, tree_path) = match kind {
            git2::ObjectType::Commit => {
                let commit = object.as_commit().unwrap();
                (commit.tree()?, "".into())
            }
            git2::ObjectType::Tree => {
                if tree_path.is_none() {
                    return Err(GitUtilsErrorCode::OtherError("tree_name and tree_path is not empty when the object is a tree".into()));
                }
                (object.into_tree().unwrap(), tree_path.unwrap())
            }
            _ => {
                return Err(GitUtilsErrorCode::OtherError(format!("Invalid Object Kind: {}", object_id)));
            }
        };

        Ok((tree, tree_path))
    }

    /// 根据提交id获取对应的文件树，object_id所指的对象应该为tree或comit，
    /// 如果是tree，则tree_name, tree_path必填，如果是commit，则会自动变为空值
    /// 
    /// tree_path的格式为：arch/alpha/boot
    pub fn get_tree_recursive(&self, object_id: Oid, tree_path: Option<String>) -> Result<fs::Dir, GitUtilsErrorCode> {
        /*
         * 
            1. GIT_FILEMODE_UNREADABLE (0o000000)
                用途：表示文件不可读。
                场景：通常用于特殊情况，比如某些文件在 Git 的索引中被标记为不可读，可能是由于权限问题或文件损坏。
            2. GIT_FILEMODE_TREE (0o040000)
                用途：表示这是一个目录（tree）。
                场景：在 Git 的树对象中，用于标识目录。Git 使用树对象来组织文件和子目录的层级结构。每个目录都有一个对应的树对象，其模式为 0o040000。
            3. GIT_FILEMODE_BLOB (0o100644)
                用途：表示一个普通文件（blob），并且具有默认的文件权限（只读）。
                场景：这是最常见的文件模式，用于普通文件。文件权限为 0o644，表示所有者有读写权限，组内用户和其他用户有读权限。
            4. GIT_FILEMODE_BLOB_GROUP_WRITABLE (0o100664)
                用途：表示一个普通文件（blob），并且具有组可写的权限。
                场景：这种模式较少见，通常用于需要组内用户可以修改文件的场景。文件权限为 0o664，表示所有者和组内用户有读写权限，其他用户有读权限。
            5. GIT_FILEMODE_BLOB_EXECUTABLE (0o100755)
                用途：表示一个可执行文件（blob）。
                场景：用于标记可执行脚本或程序。文件权限为 0o755，表示所有者有读写执行权限，组内用户和其他用户有读执行权限。
            6. GIT_FILEMODE_LINK (0o120000)
                用途：表示一个符号链接（symlink）。
                场景：符号链接是文件系统中的一个特殊文件，它指向另一个文件或目录。在 Git 中，符号链接的模式为 0o120000。
            7. GIT_FILEMODE_COMMIT (0o160000)
                用途：表示一个子模块（submodule）的提交对象。
                场景：子模块是 Git 中的一个特性，允许一个仓库嵌套另一个仓库。子模块的模式为 0o160000，表示它指向一个提交对象，而不是一个普通的文件或目录。
         */
        let (tree, tree_path) = self.resolve_tree(object_id, tree_path)?;
        // let commit = self.repository.find_commit(commit_id)?;
        // let tree = commit.tree()?;

        fn find_dir<'a>(path: &str, dir_stack: &'a mut Vec<*mut fs::Dir>) -> Option<&'a mut Dir> {
            unsafe {
                if path.is_empty() {
                    dir_stack.truncate(1);
                    return dir_stack[0].as_mut();
                } else {
                    for i in (0..dir_stack.len()).rev(){
                        let dir_path = (*dir_stack[i]).abs_path();
                        // eprintln!("dir_path: {} path: {}", dir_path, path);
                        if dir_path == path {
                            return dir_stack[i].as_mut();
                        }
                        if i != 0{
                            dir_stack.pop();
                        }
                    }
                }
                None
            }
        }
        let _path = PathBuf::from(&tree_path);
        let basename =  _path.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
        let dirname = _path.parent().and_then(|n| n.to_str()).unwrap_or_default().to_string();
        let mut root = Box::new(fs::Dir::new(dirname, basename, "0".into()));
        let mut dir_stack: Vec<*mut fs::Dir> = vec![root.as_mut() as *mut fs::Dir];
        let _ = tree.walk(TreeWalkMode::PreOrder, |path, entry| {
            /*
            拼接path，如果传入的tree_path不为None，就要拼接
            例如：tree_path为：arch/alpha/boot，path为tools/，则path为arch/alpha/boot/tools/
            如果tree_path为：arch/alpha/boot，path为""，则path为arch/alpha/boot/
            如果tree_path为""，path也为""，则path为""
            如果tree_path为""，path为"arch/"，则path为arch/
             */
            let path = if tree_path.is_empty() {
                path
            } else if path.is_empty() {
                &format!("{}/", tree_path)
            } else {
                &format!("{}/{}", tree_path, path)
            };
            let file_mode: fs::EntryMode = entry.filemode().into();
            let name = match entry.name() {
                Some(name) => name.to_string(),
                None => {
                    return TreeWalkResult::Skip
                },
            };
            let obj = match entry.to_object(&self.repository) {
                Ok(obj) => obj,
                Err(_) => return TreeWalkResult::Skip,
            };
            let object_id = obj.id().to_string();
            // eprintln!("=======================================");
            // eprintln!("path: {:?}, name: {:?}, file mode: {:?}", path, name, file_mode);
            if let Some(parent_dir) = find_dir(path, &mut dir_stack) {
                if file_mode == fs::EntryMode::Tree {
                    let dir = Box::new(fs::Dir::new(path.into(), name, object_id));
                    parent_dir.add(fs::FsNode::Dir(*dir));
                    // 从parent_dir中获取新的指针，避免重复使用Box
                    if let Some(fs::FsNode::Dir(added_dir)) = parent_dir.children.last_mut() {
                        let new_dir_ptr = added_dir as *mut fs::Dir;
                        dir_stack.push(new_dir_ptr);
                    } else {
                        eprintln!("Failed to get new directory reference for: {}", path);
                        return TreeWalkResult::Skip;
                    }
                } else {
                    let blob = match obj.into_blob() {
                        Ok(blob) => blob,
                        Err(_) => return TreeWalkResult::Skip,
                    };
                    let size = blob.size();
                    let file = Box::new(fs::File::new(path.into(), name, object_id, size, file_mode));
                    parent_dir.add(fs::FsNode::File(*file));
                }
            } else {
                // eprintln!("Failed to get parent directory for: {}", path);
                return TreeWalkResult::Skip;
            }
            TreeWalkResult::Ok
        });
        Ok(*root)
    }

    /// 根据提交获取根节点树(只获取一级)，object_id所指的对象应该为tree或comit，
    /// 如果是tree，则tree_name, tree_path必填，如果是commit，则会自动变为空值
    pub fn get_tree(&self, object_id: Oid, tree_path: Option<String>) -> Result<fs::Dir, GitUtilsErrorCode> {
        let (tree, tree_path) = self.resolve_tree(object_id, tree_path)?;
        let _path = PathBuf::from(&tree_path);
        let basename =  _path.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
        let dirname = _path.parent().and_then(|n| n.to_str()).unwrap_or_default().to_string();
        let mut root = fs::Dir::new(dirname, basename, "0".into());
        for entry in tree.iter() {
            let entry_mode: fs::EntryMode = entry.filemode().into();
            let obj = match entry.to_object(&self.repository) {
                Ok(obj) => obj,
                Err(_) => continue,
            };
            let object_id = obj.id().to_string();
            let name = match entry.name() {
                Some(name) => name.to_string(),
                None => {
                    continue;
                },
            };
            if entry_mode == fs::EntryMode::Tree {
                let dir = fs::Dir::new("".into(), name, object_id);
                root.add(fs::FsNode::Dir(dir));
            } else {
                let blob = match obj.into_blob() {
                    Ok(blob) => blob,
                    Err(_) => continue,
                };
                let size = blob.size();
                let file = fs::File::new("".into(), name, object_id, size, entry_mode);
                root.add(fs::FsNode::File(file));
            }
        }
        Ok(root)
    }
}
