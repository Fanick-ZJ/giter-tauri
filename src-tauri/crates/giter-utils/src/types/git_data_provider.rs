use std::convert::TryFrom;
use std::iter::{Chain, Flatten};
use std::path::{Path, PathBuf};
use anyhow::{Result};
use gix::status::index_worktree::iter::Item;
use gix::{refs, ObjectId, Reference, Repository};
use gix::reference::iter::Iter;
use gix::revision::Walk;
use types::progress::FuncProgress;
use types::status::FileStatus;
use log::{ error };
use types::branch::Branch;
use types::commit::Commit;
use crate::func::validate_git_repository;

pub struct GitDataProvider {
    pub repository: gix::Repository,
}

impl PartialEq<String> for GitDataProvider
{
    fn eq(&self, other: &String) -> bool {
        if other.ends_with(".git") {
            self.repository.path() == Path::new(other)
        }
        else {
            self.repository.path() == Path::new(other).join(".git")
        }
    }
}

impl GitDataProvider {
    pub fn new(repository: &str) -> Result<Self, String> {
        let repo = validate_git_repository(repository);
        match repo {
            Ok(repo) => {
                Ok(GitDataProvider { repository: repo })
            }
            Err(_) => {
                Err("INVALID GIT REPOSITORY".to_owned())
            }
        }
    }

    pub fn is_dirty(&self) -> Result<bool, String> {
        let state = self.repository.is_dirty();
        if let Ok(state) = state {
            return Ok(state);
        }

        Ok(false)
    }

    /// 获取位追踪的文件列表
    ///
    pub fn untracked_files(&self) -> Result<Vec<String>> {
        let fn_progress = FuncProgress::new("UntrackFiles", [0, 0, 0, 0]);
        let ret = self.repository.status(fn_progress)?;
        let mut untracks: Vec<String> = Vec::new();
        for entry in ret.into_index_worktree_iter(Vec::new())?.into_iter() {
            let item = entry?;
            if let Item::DirectoryContents { entry, collapsed_directory_status } = item {
                untracks.push(entry.rela_path.to_string())
            }
        }
        Ok(untracks)
    }
    ///是否有修改的文件
    ///
    pub fn has_modified_files(&self) -> Result<bool> {
        let ret = self.repository.status(FuncProgress::new("HasModifiedFiles", [0, 0, 0, 0])).unwrap();
        let iter = ret.into_index_worktree_iter(Vec::new()).unwrap().into_iter();
        for item in iter {
            if let Ok(Item::Modification {..}) = item {
                return Ok(true);
            }
        }
        Ok(false)
    }
    /// 获取修改文件的列表
    ///
    pub fn modified_files(&self) -> Result<Vec<String>> {
        let ret = self.repository.status(FuncProgress::new("ModifiedFiles", [0, 0, 0, 0])).unwrap();
        let iter = ret.into_index_worktree_iter(Vec::new())?.into_iter();
        let mut modified_files: Vec<String> = Vec::new();
        for item in iter {
            if let Ok(Item::Modification { rela_path, .. }) = item {
                modified_files.push(rela_path.to_string());
            }
        }
        Ok(modified_files)
    }
    /// 是否还未提交
    ///
    pub fn uncommit(&self) -> Result<bool> {
        let ret = self.repository.status(FuncProgress::new("Uncommit", [0, 0, 0, 0])).unwrap();
        let iter = ret.into_index_worktree_iter(Vec::new())?.into_iter();
        for item in iter {
            return Ok(true);
        }
        Ok(false)
    }
    /// 是否为推送提交
    ///
    pub fn unpushed_commits(&self) -> Result<bool> {
        let repo = &self.repository;
        let head_name = match repo.head_name()? {
            Some(name) => name.as_bstr().to_string().split('/').last().unwrap().to_string(),
            None => return Ok(false)    // Head分离的情况下，不存在分支信息
        };   // 获取当前分支名字
        let remote_names: Vec<String> = repo
            .remote_names()
            .into_iter()
            .map(|name| name.to_string())
            .collect();
        // 最近head指向分支最新的提交，
        let latest_commit_id = repo.head_commit()?.id;

        // 在所有远程分支上找，是否能找到最新的本地提交，找得到的话，就说明已经提交过了，找不到就说明远程的不是最新的
        for remote_name in remote_names {
            let ref_string = format!("refs/remotes/{}/{}", remote_name, head_name);
            if let Ok(full_name) = refs::FullName::try_from(ref_string) {
                let remote_head = repo
                    .find_reference(&full_name)?
                    .peel_to_commit()?;
                let found = remote_head.ancestors().all()?.find(|commit| commit.as_ref().unwrap().id == latest_commit_id);
                if !found.is_some() {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub fn branches(&self) -> Result<Vec<Branch>> {
        let platform = self.repository.references()?;
        let local_branches = platform.local_branches()?;
        let remote_branches = platform.remote_branches()?;
        let mut branches: Vec<Branch> = Vec::new();
        for branch in local_branches.chain(remote_branches).flatten(){
            let reference = branch.name().as_bstr().to_string();
            let is_remote = reference.starts_with("refs/remotes/");
            let basename = reference.split('/').last().unwrap().to_string();
            branches.push(Branch::new(basename, is_remote, reference));
        }
        Ok(branches)
    }

    pub fn file_status(&self) -> Result<FileStatus> {
        let path = self.repository.path().to_str().unwrap();
        match self.untracked_files() {
            Ok(untracks) => {
                if !untracks.is_empty() {
                    return Ok(FileStatus::untracked);
                }
            }
            _ => {
                error!("No untracked files found {}", path);
            }
        }
        match self.modified_files() {
            Ok(modified_files) => {
                if !modified_files.is_empty() {
                    return Ok(FileStatus::modified);
                }
            }
            _ => {
                error!("No modified files found {}", path);
            }
        }
        match self.uncommit() {
            Ok(uncommitted) => {
                if !uncommitted {
                    return Ok(FileStatus::uncommited)
                }
            }
            _ => {
                error!("No uncommitted files found {}", path);
            }
        }
        match self.unpushed_commits() {
            Ok(unpushed) => {
                if !unpushed {
                    return Ok(FileStatus::unpushed);
                }
            }
            _ => {
                error!("No unpushed commits found {}", path);
            }
        }
        Ok(FileStatus::ok)
    }

    pub fn build_commits(&self, mut revwalk: Walk, count: i32) -> Result<Box<Vec<Commit>>> {
        let mut commits: Box<Vec<Commit>> = Box::new(Vec::new());
        for i in 0..count {
            let commit_info = revwalk.next();
            if let Some(commit_info) = commit_info {
                let commit_info = commit_info?;
                let commit = self.repository.find_commit(commit_info.id())?;
                let commit = commit.decode()?;
                commits.push(
                    *Box::new(Commit::new(
                        commit_info.id.to_string(),
                        commit.author().name.to_string(),
                        commit.author().email.to_string(),
                        commit.committer().name.to_string(),
                        commit.committer().email.to_string(),
                        commit.message().summary().to_string(),
                        commit.message.to_string(),
                        commit_info.commit_time.unwrap_or_else(|| commit.time().seconds),
                        commit.parents.len() as i64,
                        self.repository.path().to_str().unwrap().to_string(),
                    ))
                );
            }
        }
        Ok(commits)
    }

    /// 从当前HEAD获取所有之前的提交
    ///
    pub fn get_commits(&self, count: i32) -> Result<Box<Vec<Commit>>> {
        let head_id = self.repository.head_id()?;
        let mut revwalk = head_id.ancestors().all()?;
        let commits = self.build_commits(revwalk, count)?;
        Ok(commits)
    }

    /// 根据commit_id 获取之前的指定数量提交
    ///
    pub fn get_commits_before(&self, commit_id:impl Into<ObjectId>, count: i32) -> Result<Box<Vec<Commit>>> {
        let commits = self.repository.find_commit(commit_id.into())?;
        let revwalk = commits.ancestors().all()?;
        let commits = self.build_commits(revwalk, count)?;
        Ok(commits)
    }

    ///获取分支的提交
    ///
    pub fn get_branch_commits(&self, branch: &Branch, count: i32) -> Result<Box<Vec<Commit>>> {
        // 获取分支所在的提交
        let commit = self.repository.find_reference(&branch.reference)?.peel_to_commit();
        let commits = commit?.ancestors().all()?;
        let commits = self.build_commits(commits, count)?;
        Ok(commits)
    }

}

#[cfg(test)]
mod tests {
    use gix::bstr::ByteSlice;
    use gix::diff::blob::intern::InternedInput;
    use gix::diff::tree_with_rewrites::Change;
    use imara_diff::{diff, Algorithm, UnifiedDiffBuilder};
    use super::*;
    #[test]
    fn test_is_dirty() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        println!("test_is_dirty: {}", provider.is_dirty().unwrap());
    }

    #[test]
    fn test_untracked_files() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        println!("test_untracked_files: {:?}", provider.untracked_files().unwrap());
    }

    #[test]
    fn test_has_modified_files() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        println!("test_has_modified_files： {}", provider.has_modified_files().unwrap());
    }

    #[test]
    fn test_modified_files() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        println!("test_modified_files： {:?}", provider.modified_files().unwrap());
    }

    #[test]
    fn test_uncommit() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        println!("test_uncommit: {}", provider.uncommit().unwrap());
    }

    #[test]
    fn test_unpushed_commits() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        println!("test_unpushed_commits: {}", provider.unpushed_commits().unwrap());
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
        let branch = Branch::new("gh-pages".to_string(), true, "refs/remotes/origin/gh-pages".to_string());
        let commits = provider.get_branch_commits(&branch,1000).unwrap();
        for commit in commits.iter() {
            println!("{:?}", commit);
        }
    }

    #[test]
    fn test_commit_content() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        let commit = provider.repository.head_commit().unwrap();
        let tree = commit.tree().unwrap();
        let prev_commit = commit.parent_ids().next().unwrap();
        let prev_commit = provider.repository.find_commit(prev_commit).unwrap();
        let prev_commit_tree = prev_commit.tree().unwrap();
        let diff = provider.repository.diff_tree_to_tree(&prev_commit_tree, &tree, gix::diff::Options::default()).unwrap();
        for change in diff.iter() {
            match change {
                Change::Addition { .. } => {}
                Change::Deletion { .. } => {}
                Change::Modification { id, previous_id, .. } => {
                    let old_content = provider.repository.find_blob(*previous_id).unwrap();
                    let new_content = provider.repository.find_blob(*id).unwrap();
                    println!("-----------------------old--------------------------------");
                    println!("{}", old_content.data.to_str().unwrap());
                    println!("------------------------new-------------------------------");
                    println!("{}", new_content.data.to_str().unwrap());
                }
                Change::Rewrite { .. } => {}
            }
        }
    }

    #[test]
    fn test_blob_diff() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        let commit = provider.repository.head_commit().unwrap();
        let tree = commit.tree().unwrap();
        let prev_commit = commit.parent_ids().next().unwrap();
        let prev_commit = provider.repository.find_commit(prev_commit).unwrap();
        let prev_commit_tree = prev_commit.tree().unwrap();
        let diff_tree = provider.repository.diff_tree_to_tree(&prev_commit_tree, &tree, gix::diff::Options::default()).unwrap();
        for change in diff_tree.iter() {
            match change {
                Change::Addition { .. } => {}
                Change::Deletion { .. } => {}
                Change::Modification { id, previous_id, .. } => {
                    let old_content = provider.repository.find_blob(*previous_id).unwrap();
                    let new_content = provider.repository.find_blob(*id).unwrap();
                    let input = InternedInput::new(
                        old_content.data.to_str().unwrap(),
                        new_content.data.to_str().unwrap());
                    let alg = Algorithm::Histogram;
                    let diff = diff(alg, &input, UnifiedDiffBuilder::new(&input));
                    println!("diff {}", diff);
                }
                Change::Rewrite { .. } => {}
            }
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