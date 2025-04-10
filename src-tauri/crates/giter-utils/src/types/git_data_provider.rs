use crate::util::build_commit;
use crate::util::change_status_to_file_status;
use crate::util::get_file_content;
use crate::util::is_binary_file;
use crate::util::is_binary_file_content;
use crate::util::size_by_path;
use crate::util::stamp_to_ymd;
use crate::util::write_file;
use anyhow::anyhow;
use anyhow::Result;
use git2::build::CheckoutBuilder;
use git2::Cred;
use git2::CredentialType;
use git2::Delta;
use git2::FetchOptions;
use git2::PushOptions;
use git2::RemoteCallbacks;
use git2::TreeWalkMode;
use git2::{BranchType, Oid, Repository, Revwalk, Status};
use serde_json::Value;
use similar::DiffOp;
use similar::TextDiff;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Pointer;
use std::i32;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::usize;
use std::vec;
use super::{author::Author, branch::Branch, commit::Commit, status::WorkStatus};

use super::cache::Cache;
use super::commit_filter::FilterConditions;
use super::contribution::CommitStatistic;
use super::credential::Credential;
use super::diff::ContentDiff;
use super::file::ChangedFile;
use super::file::CommittedFile;
use super::file::UntrackedFile;
use super::status::status_to_changed_status;
use super::status::FileStatus;
use super::error::ErrorCode as GitError;


pub struct GitDataProvider {
    pub repository: Repository,
    cache: RefCell<Option<Box<dyn Cache + Send>>>,
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
    pub fn new<P: AsRef<Path>>(repository: P) -> Result<Self, GitError> {
        let repo = Repository::open(&repository);
        match repo {
            Ok(repo) => Ok(GitDataProvider {
                repository: repo,
                cache: RefCell::new(None),
            }),
            Err(err) => {
                match err.code() {
                    git2::ErrorCode::NotFound => Err(GitError::RepoNotFound(repository.as_ref().display().to_string())),
                    git2::ErrorCode::Owner => Err(GitError::NoOwner(repository.as_ref().display().to_string())),
                    _ => Err(GitError::OtherError(err.message().to_string())),
                }
            }
        }
    }

    pub fn set_cache(&mut self, cache: impl Cache + Send + 'static) {
        self.cache = RefCell::new(Some(Box::new(cache)));
    }

    pub fn workdir(&self) -> &Path {
        self.repository.workdir().unwrap()
    }

    pub fn author(&self) -> Result<Author, GitError> {
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
    pub fn untracked_files(&self) -> Result<Vec<UntrackedFile>, GitError> {
        let status = self.repository.statuses(None)?;
        let mut untracks: Vec<UntrackedFile> = Vec::new();
        for item in &status {
            if item.status() == Status::WT_NEW {
                let abs_path = self.blob_path(item.path().unwrap());
                let size = size_by_path(&abs_path)? as usize;
                let is_binary = is_binary_file(&abs_path)
                    .map_err(|e| GitError::OtherError(e.to_string()))?;
                let untracked_file = UntrackedFile::new(&abs_path, size, is_binary);
                untracks.push(untracked_file);
            }
        }
        Ok(untracks)
    }

    ///工作空间是否有修改
    ///
    pub fn workspace_change(&self) -> Result<Vec<String>, GitError> {
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

    pub fn uncommitted(&self) -> Result<bool, GitError> {
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

    fn blob_is_binary(&self, oid: Oid) -> Result<bool, GitError> {
        let blob = self.repository
            .find_blob(oid)
            .map_err(|_| GitError::BlobNotFound(oid.to_string()))?;
        let content = blob.content();
        let is = is_binary_file_content(content.to_vec());
        Ok(is)

    }

    pub fn staged_files(&self) -> Result<Vec<ChangedFile>, GitError> {
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

    pub fn changed_files(&self) -> Result<Vec<ChangedFile>, GitError> {
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

    pub fn add_to_stage(&self, path: &PathBuf) -> Result<(), GitError> {
        let repo = &self.repository;
        let mut index = repo.index()?;
        // TODO:还差判断gitignore
        index.add_path(&path)?;
        index.write()?;
        Ok(()) 
    }

    pub fn remove_from_stage(&self, path: &PathBuf) -> Result<(), GitError> {
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
                    return Err(GitError::OtherError("File not found in index".to_string()));
                }
            } 
        }
        index.write()?;
        Ok(())
    }

    /// 暂存区文件恢复到工作区
    pub fn checkout_file (&self, path: &PathBuf) -> Result<(), GitError> {
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

    pub fn current_branch(&self) -> Result<Branch, GitError> {
        let repo = &self.repository;
        let branches = repo.branches(None).unwrap();
        for branch in branches {
            let (branch, _) = branch.unwrap();
            if branch.is_head() {
                return Ok(Branch::from(branch.into_reference()));
            }
        }
        Err(GitError::CurrentBranchNotFound("No current branch found".to_string()))
    }

    /// 获取分支列表
    pub fn branches(&self) -> Result<Vec<Branch>, GitError> {
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
    pub fn current_remote_branch(&self) -> Result<Branch, GitError>{
        let current = self.current_branch()?;
        let remote = self
            .repository
            .find_branch(&current.reference, BranchType::Local)?
            .upstream();
        match remote {
            Ok(remote) => {
                Ok(Branch::from(remote.into_reference()))
            }
            Err(_) => Err(GitError::RemoteNotFound(current.name)),
        }
    }

    fn current_remote_branch_inner(&self) -> Result<git2::Branch, GitError> {
        let current = self.current_branch()?;
        let remote = self
            .repository
            .find_branch(&current.reference, BranchType::Local)?
            .upstream();
        match remote {
            Ok(remote) => Ok(remote),
            Err(_) => Err(GitError::RemoteNotFound(current.name)),
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
    pub fn work_status(&self) -> Result<WorkStatus, GitError> {
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

    pub fn get_branch(&self, branch_name: &str) -> Result<Branch, GitError> {
        let branches = self.branches()?;
        for branch in branches {
            if branch.name == branch_name {
                return Ok(branch);
            }
        }
        Err(GitError::BranchNotFound(branch_name.to_string()))
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
    fn branch_commit_inner(&self, branch: &Branch) -> Result<git2::Commit, GitError> {
        let b_type = if branch.is_remote {
            BranchType::Remote
        } else {
            BranchType::Local
        };
        println!("branch: {:?}", branch);
        let branch_reference = self.repository.find_branch(&branch.name, b_type);
        if let Err(_) = branch_reference {
            return Err(GitError::BranchNotFound(branch.name.clone()));
        }
        let branch_reference = branch_reference?;
        let branch_commit = branch_reference.get().peel_to_commit();
        if let Err(_) = branch_commit {
            return Err(GitError::BranchNotFound(branch.name.clone()));
        }
        Ok(branch_commit?)
    }

    /// 获取分支的提交，返回提交对象和此分支的提交总数
    ///
    pub fn get_branch_commits(&self, branch: &Branch, count: i32) -> Result<Vec<Commit>, GitError> {
        // 获取分支所在的提交
        let commit = self.branch_commit_inner(branch)?;
        let mut revwalk = self.repository.revwalk()?;
        revwalk.push(commit.id())?;
        let commits = self.build_commits(&mut revwalk, count)?;
        Ok(commits)
    }

    fn tree_walk(&self, tree: &git2::Tree) -> Vec<CommittedFile> {
        let mut files: Vec<CommittedFile> = Vec::new();
        let _ = tree.walk(TreeWalkMode::PostOrder, |root, entry| {
            let path = Path::new(root);
            let path = path.join(entry.name().unwrap()).to_str().unwrap().to_string();
            let blob = self.repository.find_blob(entry.id());
            if blob.is_err() {
                files.push(CommittedFile::new(path, 0, FileStatus::Added, "0".repeat(20), "0".repeat(20), false, false, false));
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

    pub fn get_commit (&self, commit_id: impl Into<Oid>) -> Result<Commit, GitError> {
        let commit = self.repository.find_commit(commit_id.into())?;
        let commit = build_commit(&commit, &self.repository);
        Ok(commit)
    }

    /// 获取一个提交的内容
    ///
    pub fn commit_content(&self, commit_id: impl Into<Oid>) -> Result<Vec<CommittedFile>, GitError> {
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
            let (is_binary, old_is_binary) = (
                self.blob_is_binary(new_file.id())?,
                self.blob_is_binary(old_file.id())?,
            );
            let size = new_file.size();
            let status = change_status_to_file_status(&delta.status());
            let new_blob = repo.find_blob(delta.new_file().id());
            let new_id = new_file.id().to_string();
            let old_id = old_file.id().to_string();
            let (exist, _) = match new_blob {
                Ok(blob) => (true, blob.content().to_vec()),
                Err(_) => (false, Vec::new()),
            };
            let file = CommittedFile::new(path, size as usize, status, new_id, old_id, exist, is_binary, old_is_binary);
            files.push(file);
        }
        Ok(files)
    }

    /// 根据oid获取文件内容
    /// oid: 提交id
    pub fn get_blob_content(&self, oid: impl Into<Oid>) -> Result<Vec<u8>, GitError> {
        let oid = oid.into();
        let blob = self.repository.find_blob(oid);
        if blob.is_err() {
            return Err(GitError::BlobNotFound(oid.to_string()));
        }
        let blob = blob.unwrap();
        Ok(blob.content().to_vec())
    }

    /// 获取文件的差异
    /// old: 旧的文件内容
    /// new: 新的文件内容
    ///
    pub fn get_file_diff(&self, old: impl Into<Oid>, new: impl Into<Oid>) -> Result<(Vec<DiffOp>, String), GitError> {
        let old = old.into();
        let new = new.into();
        let old_blob = self.repository.find_blob(old);
        let new_blob = self.repository.find_blob(new);
        if old_blob.is_err() {
            return Err(GitError::BlobNotFound(old.to_string()));
        }
        if new_blob.is_err() {
            return Err(GitError::BlobNotFound(new.to_string()));
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
    pub fn get_file_content_diff(&self, old: impl Into<Oid>, new: impl Into<Oid>) -> Result<ContentDiff, GitError> {
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
    pub fn authors(&self, branch: &Branch) -> Result<Vec<Author>, GitError> {
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
        let lasted_id = branch_commit.id();
        let authors: Vec<Author> = author_set.into_iter().collect();
        // 3. 设置缓存
        self.set_authors_cache(authors.clone(), branch, &lasted_id);
        Ok(authors)
    }

    
    pub fn get_branch_commit_contribution(&self, branch: &Branch) -> Result<Vec<CommitStatistic>> {
        let authors = self.authors(branch)?;
        let commits = self.branch_commit_inner(branch)?;
        let mut revwalk = self.repository.revwalk()?;
        revwalk.push(commits.id())?;
        let mut map = HashMap::<String, CommitStatistic>::new();
        let cache = self.branch_contribution_cache(branch);
        if let Some(cache) = cache {
            map.extend(cache.0);
            revwalk.hide(cache.1)?;
        }
        for commit in revwalk.by_ref().into_iter() {
            if let Ok(commit) = commit {
                let commit = self.repository.find_commit(commit)?;
                let author = commit.author();
                let email = author.email().unwrap().to_string();
                let time = stamp_to_ymd(commit.time().seconds());
                if let Err(_) = time {
                    continue;
                }
                let time = time.unwrap();
                if !map.contains_key(email.as_str()) {
                    let author = authors.iter().find(|a| a.email == email);
                    if let None = author {
                        continue;
                    }
                    let author = author.unwrap().clone();
                    map.insert(email.clone(), CommitStatistic::new(self.workdir().to_path_buf(), branch.clone(), author));      
                }
                let stat = map.get_mut(email.as_str()).unwrap();
                let _ = stat.add(time, 1);
            } else {
                continue;
            }
        }
        // 获取最后一次提交的id
        let lasted_id = commits.id();
        self.set_branch_contribution_cache(branch, &map, &lasted_id);
        Ok(map.into_values().collect())
    }

    pub fn get_branch_commits_after_filter(&self, branch: &Branch, filter: &HashMap<String, Value>) -> Result<Vec<Commit>, GitError> {
        let branch_commits = self.get_branch_commits(branch, i32::MAX)?;
        let filter = FilterConditions::build_from_sv_map(filter);
        // 获取过滤条件
        let last_id= filter.last_id.unwrap_or_default();
        let mut commits = Vec::<Commit>::new();
        // 遍历提交
        for commit in branch_commits.into_iter() {
            if commit.commit_id == last_id || commits.len() >= filter.count {
                break; 
            }
            let author = &filter.author;
            let time = commit.datetime;
            if (!author.is_default() && author.email != commit.author_email && author.name != commit.author_name)
                || time < filter.start_time || time > filter.end_time {
                continue; 
            }
            commits.push(commit);
        }
        Ok(commits)
    }
 
    pub fn commit (&self, message: &str, update_ref: Option<&str>) -> Result<Oid, GitError> {
        // 如果没有指定更新的分支，默认更新当前分支
        let update_ref =  if update_ref.is_none() {
           Some("HEAD")
        } else  {
            update_ref
        };
        let repo = &self.repository;
        if repo.is_bare() {
            return Err(GitError::RepoIsBare(repo.path().display().to_string()));
        }
        let mut index = repo.index()?;
        let conflicts = index.conflicts()?;
        if conflicts.into_iter().count() > 0 {
            return Err(GitError::RepoHasConflicts(repo.path().display().to_string())); 
        }
        let author = repo.signature();
        if author.is_err() {
            return Err(GitError::RepoAuthorNoConfig);
        }
        if self.staged_files()?.len() == 0 {
            return Err(GitError::NoStagedFile);
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

    fn build_remote_credentials_cb(&self, credentials: &Option<(String, String)>, use_cache: Rc<RefCell<bool>>) -> impl Fn(&str, Option<&str>, CredentialType) -> Result<Cred, git2::Error> + '_ {
        let (username, password) = credentials.as_ref()
            .map(|(u, p)| (u.clone(), p.clone()))
            .unwrap_or_default();
        move |url: &str, username_from_url: Option<&str>, allowed_types: CredentialType| {
            let cache = self.remote_credential(url);
            if let Some(cache) = cache {
               match cache {
                    Credential::UsernamePassword(username, password) => {
                        *use_cache.borrow_mut() = false;
                        return Cred::userpass_plaintext(&username, &password);
                    }
                    Credential::Token(_) => todo!(),
                } 
            }
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


    pub fn push(&self, remote: &str, branch_name: &str, credentials: Option<(String, String)>) -> Result<(), GitError> {
        let repo = &self.repository;
        
        // 提取远程获取和分支验证逻辑
        let mut remote = repo.find_remote(remote)
            .map_err(|_| GitError::RemoteNotFound(branch_name.to_string()))?;

        let branch = repo.find_branch(branch_name, BranchType::Local)
            .map_err(|_| GitError::BranchNotFound(branch_name.to_string()))?;
        
        // 验证分支跟踪状态
        if !self.has_tracking(&branch) {
            return Err(GitError::BranchNotTrackAny(branch_name.to_string()));
        }

        // 提取公共回调配置
        let use_cache = Rc::new(RefCell::new(true));
        let build_callbacks = || {
            let mut cbs = RemoteCallbacks::new();
            cbs.credentials(self.build_remote_credentials_cb(&credentials, use_cache.clone()));
            cbs
        };

        // 统一错误处理函数
        let handle_error = |e: git2::Error| {
            log::error!("Git operation error: {:?}", e);
            match e.code() {
                git2::ErrorCode::User => GitError::PushNeedNameAndPassword,
                _ => return GitError::PushOtherError.into()
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
            return Err(GitError::RemoteHeadHasNotInLocal.into());
        }

        // 执行push操作
        let mut push_opt = PushOptions::new();
        push_opt.remote_callbacks(build_callbacks());
        remote.push(&[branch_ref_name], Some(&mut push_opt))
            .map_err(handle_error)?;
        // 设置远程凭据缓存
        if credentials.is_some() && *use_cache.borrow() {
            let (username, password) = credentials.unwrap();
            self.set_remote_credential(remote.url().unwrap(), &Credential::UsernamePassword(username, password));
        }
        println!("Push successful!");
        Ok(())
    }

    pub fn pull(&self, remote: &str, branch: &str, credentials: Option<(String, String)>) -> Result<(), GitError> {
        let repo = &self.repository;
    
        // 1. 获取远程引用
        let mut remote = repo.find_remote(remote)
            .map_err(|e| {
                log::error!("Find remote error: {:?}", e);
                GitError::RemoteNotFound(remote.to_string())
            })?;
        // 2. 配置回调（复用已有的凭证处理逻辑）
        let use_cache = Rc::new(RefCell::new(true));
        let callbacks = self.build_remote_credentials_cb(&credentials, use_cache.clone());
        // 3. 执行 fetch 操作
        let mut fetch_opts = FetchOptions::new();
        fetch_opts.remote_callbacks({
            let mut cb = RemoteCallbacks::new();
            cb.credentials(callbacks);
            cb
        });
        // 获取本地分支
        let local_branch = repo.find_branch(branch, BranchType::Local)
            .map_err(|_| GitError::BranchNotFound(branch.to_string()))?;
        let local_branch_ref = local_branch.into_reference();

        // 4. 获取FETCH_HEAD 提交
        remote.fetch(&[local_branch_ref.name().unwrap()], Some(&mut fetch_opts), None)
           .map_err(|e| {
            log::error!("Fetch error: {:?}", e);
            GitError::PushOtherError
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
                GitError::TargetReferenceNotDirect
            })?;
            repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
            return Ok(());
        }
        else if analysis.0.is_normal() {
            // 创建提交的合并
            let head_commit = repo.head()?.peel_to_commit()?;
            repo.merge(&[&fetch_commit], None, None).map_err(|e| {
                log::error!("Merge error: {:?}", e);
                GitError::CommitBeforePullWouldBeOverwrittenByMerge
            })?;
            // 创建合并提交
            let signature = repo.signature().map_err(|_| GitError::UserUnConfigured)?;
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
                    GitError::BuildMergeCommitError
                })?;
        }
        else if analysis.0.is_none() {
            return Err(GitError::CantPull.into());
        }
        if credentials.is_some() && *use_cache.borrow() {
            let (username, password) = credentials.unwrap();
            self.set_remote_credential(remote.url().unwrap(), &Credential::UsernamePassword(username, password));
        }
        Ok(())
    }

    /// 检查两个分支是否存在合并冲突
    pub fn check_branch_conflict(&self, branch_a: impl Into<Branch>, branch_b: impl Into<Branch>) -> Result<bool, GitError> {
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
    pub fn switch_branch(&self, branch: &Branch) -> Result<(), GitError> {
        let staged = self.staged_files()?;
        if staged.len() > 0 {
            return Err(GitError::OtherError("There are staged files, please commit them first".to_string())); 
        }
        let repo = &self.repository;
        let branch_name = branch.name.to_string();

        // 获取目标分支的树对象
        let target_commit = self.branch_commit_inner(branch)?;
        let target_tree = target_commit.tree()?;
        let branch = repo.find_branch(&branch_name, BranchType::Local)
           .map_err(|_| GitError::BranchNotFound(branch_name.clone()))?;
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
            return Err(GitError::SwitchWillBeOverwrittenByMerge(conflicts.join("\n"))); 
        }
        let mut blob_map = HashMap::new();
        // 获取修改的文件内容，在切换之后再恢复
        for file in work_changed {
           let abs_path = self.blob_path(file); 
           let blob = get_file_content(&abs_path)?;
           blob_map.insert(abs_path, blob);
        }
        repo.set_head(branch_ref_name)
          .map_err(|e| GitError::Git2Error(e))?;
        let _ = repo.checkout_head(Some(CheckoutBuilder::default().force()));
        // 恢复修改的文件内容
        for (path, blob) in blob_map {
            let _ = write_file(&path, &blob);
        }
        Ok(())
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

    /// 获取分支的贡献者统计
    /// 
    pub fn branch_contribution_cache(&self, branch: &Branch) -> Option<(HashMap<String, CommitStatistic>, Oid)> {
        if let Some(cache) = self.cache.borrow().as_ref() {
            let contrib = cache.branch_contribution(self.repository.path().to_str().unwrap(), branch);
            if let Some((contrib, last_commit_id)) = contrib {
                return Some((contrib, last_commit_id));
            }
        }
        None
    }
    /// 设置分支的贡献者统计
    /// 
    pub fn set_branch_contribution_cache(
        &self,
        branch: &Branch,
        contrib: &HashMap<String, CommitStatistic>,
        last_commit_id: &Oid,
    ) {
        if let Some(cache) = self.cache.borrow_mut().as_mut() {
            let path = self.repository.path().to_str().unwrap();
            cache.set_branch_contribution(path, branch, contrib, last_commit_id);
        }
    }
    /// 设置提交者缓存
    ///
    pub fn set_authors_cache(&self, authors: Vec<Author>, branch: &Branch, lasted_id: &Oid) {
        if let Some(cache) = self.cache.borrow_mut().as_mut() {
            cache.set_authors(
                self.repository.path().to_str().unwrap(),
                &authors,
                branch,
                lasted_id,
            );
        }
    }

    pub fn remote_credential(&self, remote_url: &str) -> Option<Credential> {
        if let Some(cache) = self.cache.borrow().as_ref() {
            return cache.get_credential(remote_url);
        }
        None
    }

    pub fn set_remote_credential(&self, remote_url: &str, credential: &Credential) {
        if let Some(cache) = self.cache.borrow_mut().as_mut() {
            cache.set_credential(remote_url, credential);
        } 
    }
}

#[cfg(test)]
mod tests {

    use similar::{ChangeTag, TextDiff};

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
    fn test_get_author() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        println!("test_get_author: {:?}", provider.author().unwrap());
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

    #[test]
    fn test_diff() {
        let diff = TextDiff::from_lines(
            "Hello World\nThis is the second line.\nMoar and more",
            "Hallo Welt\nThis is the second line.\nThis is the third.\nMoar and more",
        );
        diff.ops().iter().for_each(| f | {
            println!("----{:?}", f);
        });
    }
}
