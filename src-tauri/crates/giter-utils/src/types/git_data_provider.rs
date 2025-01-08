use crate::func::validate_git_repository;
use crate::types::status::FileStatus;
use crate::util::{build_commit, build_file_between_tree, get_blob_size};
use anyhow::Result;
use gix::objs::tree::EntryKind;
use gix::revision::Walk;
use gix::status::index_worktree::iter::Item;
use gix::{refs, ObjectId};
use log::error;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::path::Path;
use types::{
    author::Author, branch::Branch, commit::Commit, file::File, progress::FuncProgress,
    status::WorkStatus,
};

pub struct GitDataProvider {
    pub repository: gix::Repository,
}

impl PartialEq<String> for GitDataProvider {
    fn eq(&self, other: &String) -> bool {
        if other.ends_with(".git") {
            self.repository.path() == Path::new(other)
        } else {
            self.repository.path() == Path::new(other).join(".git")
        }
    }
}

impl GitDataProvider {
    pub fn new(repository: &str) -> Result<Self, String> {
        let repo = validate_git_repository(repository);
        match repo {
            Ok(repo) => Ok(GitDataProvider { repository: repo }),
            Err(_) => Err("INVALID GIT REPOSITORY".to_owned()),
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
            if let Item::DirectoryContents {
                entry,
                collapsed_directory_status,
            } = item
            {
                untracks.push(entry.rela_path.to_string())
            }
        }
        Ok(untracks)
    }
    ///是否有修改的文件
    ///
    pub fn has_modified_files(&self) -> Result<bool> {
        let ret = self
            .repository
            .status(FuncProgress::new("HasModifiedFiles", [0, 0, 0, 0]))
            .unwrap();
        let iter = ret
            .into_index_worktree_iter(Vec::new())
            .unwrap()
            .into_iter();
        for item in iter {
            if let Ok(Item::Modification { .. }) = item {
                return Ok(true);
            }
        }
        Ok(false)
    }
    /// 获取修改文件的列表
    ///
    pub fn modified_files(&self) -> Result<Vec<String>> {
        let ret = self
            .repository
            .status(FuncProgress::new("ModifiedFiles", [0, 0, 0, 0]))
            .unwrap();
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
        let ret = self
            .repository
            .status(FuncProgress::new("Uncommit", [0, 0, 0, 0]))
            .unwrap();
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
            Some(name) => name
                .as_bstr()
                .to_string()
                .split('/')
                .last()
                .unwrap()
                .to_string(),
            None => return Ok(false), // Head分离的情况下，不存在分支信息
        }; // 获取当前分支名字
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
                let remote_head = repo.find_reference(&full_name)?.peel_to_commit()?;
                let found = remote_head
                    .ancestors()
                    .all()?
                    .find(|commit| commit.as_ref().unwrap().id == latest_commit_id);
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
        for branch in local_branches.chain(remote_branches).flatten() {
            let reference = branch.name().as_bstr().to_string();
            let is_remote = reference.starts_with("refs/remotes/");
            let basename = reference.split('/').last().unwrap().to_string();
            branches.push(Branch::new(basename, is_remote, reference));
        }
        Ok(branches)
    }

    pub fn file_status(&self) -> Result<WorkStatus> {
        let path = self.repository.path().to_str().unwrap();
        match self.untracked_files() {
            Ok(untracks) => {
                if !untracks.is_empty() {
                    return Ok(WorkStatus::Untracked);
                }
            }
            _ => {
                error!("No untracked files found {}", path);
            }
        }
        match self.modified_files() {
            Ok(modified_files) => {
                if !modified_files.is_empty() {
                    return Ok(WorkStatus::Modified);
                }
            }
            _ => {
                error!("No modified files found {}", path);
            }
        }
        match self.uncommit() {
            Ok(uncommitted) => {
                if !uncommitted {
                    return Ok(WorkStatus::Uncommited);
                }
            }
            _ => {
                error!("No uncommitted files found {}", path);
            }
        }
        match self.unpushed_commits() {
            Ok(unpushed) => {
                if !unpushed {
                    return Ok(WorkStatus::Unpushed);
                }
            }
            _ => {
                error!("No unpushed commits found {}", path);
            }
        }
        Ok(WorkStatus::Ok)
    }

    pub fn build_commits(&self, mut revwalk: Walk, count: i32) -> Result<Box<Vec<Commit>>> {
        let mut commits: Box<Vec<Commit>> = Box::new(Vec::new());
        for i in 0..count {
            let commit_info = revwalk.next();
            if let Some(commit_info) = commit_info {
                let commit_info = commit_info?;
                let commit = self.repository.find_commit(commit_info.id())?;
                commits.push(*Box::new(build_commit(&commit)));
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
    pub fn get_commits_before(
        &self,
        commit_id: impl Into<ObjectId>,
        count: i32,
    ) -> Result<Box<Vec<Commit>>> {
        let commits = self.repository.find_commit(commit_id.into())?;
        let revwalk = commits.ancestors().all()?;
        let commits = self.build_commits(revwalk, count)?;
        Ok(commits)
    }

    /// 获取分支的提交
    ///
    pub fn get_branch_commits(&self, branch: &Branch, count: i32) -> Result<Box<Vec<Commit>>> {
        // 获取分支所在的提交
        let commit = self
            .repository
            .find_reference(&branch.reference)?
            .peel_to_commit();
        let commits = commit?.ancestors().all()?;
        let commits = self.build_commits(commits, count)?;
        Ok(commits)
    }

    /// 获取一个提交的内容
    ///
    pub fn get_commit_content(&self, commit_id: impl Into<ObjectId>) -> Result<Vec<File>> {
        let commit = self.repository.find_commit(commit_id.into());
        if let Err(_) = commit {
            return Err(anyhow::anyhow!("Commit not found"));
        }
        let commit = commit?;
        let tree = commit.tree()?;
        let mut files: Vec<File> = Vec::new();
        // 获取父提交
        let parent = commit.parent_ids().into_iter().next();
        if let Some(parent) = parent {
            let parent = self.repository.find_commit(parent)?;
            let parent_tree = parent.tree()?;
            let files = build_file_between_tree(&self.repository, &parent_tree, &tree);
            return Ok(files);
        } else {
            let tree = self.repository.find_tree(tree.id).unwrap();
            tree.iter().for_each(|f| {
                let f = f.unwrap();
                let (exist, size) = get_blob_size(&self.repository, f.id());
                files.push(File::new(
                    f.filename().to_string(),
                    size,
                    FileStatus::Modified,
                    f.id().to_string(),
                    EntryKind::from(f.mode()),
                    "".to_string(),
                    exist,
                ));
            });
        }
        Ok(files)
    }

    pub fn get_contributors(&self, branch: &Branch) -> Result<Vec<Author>> {
        // 获取所有贡献者
        // 1. 获取分支的提交
        let branch_reference = self.repository.find_reference(&branch.reference);
        if let Err(_) = branch_reference {
            return Err(anyhow::anyhow!("Branch not found"));
        }
        let mut branch_reference = branch_reference?;
        let branch_commit = branch_reference.peel_to_commit();
        if let Err(_) = branch_commit {
            return Err(anyhow::anyhow!("Branch not found"));
        }
        let branch_commit = branch_commit?;
        let mut author_set = HashSet::new();
        // 2. 获取提交的作者, 获取作者的邮箱
        for commit in branch_commit.ancestors().all()? {
            let commit = commit?;
            let commit_obj = commit.object()?;
            let author = commit_obj.author().unwrap();
            author_set.insert(Author::new(
                author.name.to_string(),
                author.email.to_string(),
            ));
        }
        Ok(author_set.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {

    use std::io::Write;

    use super::*;
    use gix::bstr::ByteSlice;
    use gix::diff::blob::intern::InternedInput;
    use gix::diff::tree_with_rewrites::Change;
    use imara_diff::{diff, Algorithm, UnifiedDiffBuilder};
    #[test]
    fn test_is_dirty() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        println!("test_is_dirty: {}", provider.is_dirty().unwrap());
    }

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
            "test_has_modified_files： {}",
            provider.has_modified_files().unwrap()
        );
    }

    #[test]
    fn test_modified_files() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        println!(
            "test_modified_files： {:?}",
            provider.modified_files().unwrap()
        );
    }

    #[test]
    fn test_uncommit() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        println!("test_uncommit: {}", provider.uncommit().unwrap());
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

    // #[test]
    // fn test_commit_content() {
    //     let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
    //     let commit = provider.repository.head_commit().unwrap();
    //     println!("{:?}", commit.id.to_string());
    //     let tree = commit.tree().unwrap();
    //     let prev_commit = commit.parent_ids().next().unwrap();
    //     let prev_commit = provider.repository.find_commit(prev_commit).unwrap();
    //     let prev_commit_tree = prev_commit.tree().unwrap();
    //     let diff = provider.repository.diff_tree_to_tree(&prev_commit_tree, &tree, gix::diff::Options::default()).unwrap();
    //     for change in diff.iter() {
    //         match change {
    //             Change::Addition { .. } => {}
    //             Change::Deletion { .. } => {}
    //             Change::Modification { id, previous_id, .. } => {
    //                 let old_content = provider.repository.find_blob(*previous_id).unwrap();
    //                 let new_content = provider.repository.find_blob(*id).unwrap();
    //                 // println!("-----------------------old--------------------------------");
    //                 // println!("{}", old_content.data.to_str().unwrap());
    //                 // println!("------------------------new-------------------------------");
    //                 // println!("{}", new_content.data.to_str().unwrap());
    //             }
    //             Change::Rewrite { .. } => {}
    //         }
    //     }
    // }

    #[test]
    fn test_blob_diff() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        let commit = provider.repository.head_commit().unwrap();
        let tree = commit.tree().unwrap();
        let prev_commit = commit.parent_ids().next().unwrap();
        let prev_commit = provider.repository.find_commit(prev_commit).unwrap();
        let prev_commit_tree = prev_commit.tree().unwrap();
        let diff_tree = provider
            .repository
            .diff_tree_to_tree(&prev_commit_tree, &tree, gix::diff::Options::default())
            .unwrap();
        for change in diff_tree.iter() {
            match change {
                Change::Addition { .. } => {}
                Change::Deletion { .. } => {}
                Change::Modification {
                    id, previous_id, ..
                } => {
                    let old_content = provider.repository.find_blob(*previous_id).unwrap();
                    let new_content = provider.repository.find_blob(*id).unwrap();
                    let input = InternedInput::new(
                        old_content.data.to_str().unwrap(),
                        new_content.data.to_str().unwrap(),
                    );
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

    #[test]
    fn test_commit_content() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL").unwrap();
        let obj_id =
            ObjectId::from_hex("6fc635a95b94c0f59236505ce8977bbc4c235f9f".as_bytes()).unwrap();
        let commit = provider.get_commit_content(obj_id);
        // println!("{:?}", &commit.unwrap().len());
        for file in commit.unwrap() {
            println!("{:?}, {}", file.status, file.path);
        }
    }

    #[test]
    fn test_get_all_commit() {
        let provider =
            GitDataProvider::new(r"E:\workSpace\Python_Project_File\wizvision3").unwrap();
        let commits = provider.get_commits(1000000).unwrap();
        let save_path = r"C:\Users\ZJFan\OneDrive\桌面\commit.txt";
        let mut f = std::fs::File::create(save_path).unwrap();
        for commit in commits.iter() {
            // println!("{:?}", commit);
            let obj_id = ObjectId::from_hex(commit.commit_id.as_bytes()).unwrap();
            let files = provider.get_commit_content(obj_id).unwrap();
            f.write(
                format!(
                    "-------------------{}--------------------\n",
                    commit.commit_id
                )
                .as_bytes(),
            )
            .unwrap();
            for file in files.iter() {
                // println!("{:?}", file);
                f.write(format!("{}\n", file.path).as_bytes()).unwrap();
            }
        }
    }

    #[test]
    fn test_get_contributors() {
        let provider =
            GitDataProvider::new(r"E:\workSpace\Python_Project_File\wizvision3").unwrap();
        let branchs = provider.branches().unwrap();
        for branch in branchs.iter() {
            println!("{:?}", branch);
            let contributors = provider.get_contributors(branch).unwrap();
            for contributor in contributors.iter() {
                println!("{:?}", contributor);
            }
        }
    }
}
