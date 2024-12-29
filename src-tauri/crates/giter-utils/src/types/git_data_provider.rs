use std::convert::TryFrom;
use gix::status::index_worktree::iter::Item;
use gix::refs;
use types::progress::FuncProgress;
use crate::func::validate_git_repository;

pub struct GitDataProvider {
    pub repository: gix::Repository,
}

impl GitDataProvider {
    pub fn new(repository: &str) -> Self {
        let repo = validate_git_repository(repository).unwrap();
        Self {
            repository: repo,
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
    pub fn untracked_files(&self) -> Result<Vec<String>, String> {
        let fn_progress = FuncProgress::new("UntrackFiles", [0, 0, 0, 0]);
        let ret = self.repository.status(fn_progress).unwrap();
        let mut untracks: Vec<String> = Vec::new();
        for entry in ret.into_index_worktree_iter(Vec::new()).unwrap().into_iter() {
            let item = entry.unwrap();
            if let Item::DirectoryContents { entry, collapsed_directory_status } = item {
                untracks.push(entry.rela_path.to_string())
            }
        }
        Ok(untracks)
    }
    ///是否有修改的文件
    ///
    pub fn has_modified_files(&self) -> Result<bool, String> {
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
    pub fn modified_files(&self) -> Result<Vec<String>, String> {
        let ret = self.repository.status(FuncProgress::new("ModifiedFiles", [0, 0, 0, 0])).unwrap();
        let iter = ret.into_index_worktree_iter(Vec::new()).unwrap().into_iter();
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
    pub fn uncommit(&self) -> Result<bool, String> {
        let ret = self.repository.status(FuncProgress::new("Uncommit", [0, 0, 0, 0])).unwrap();
        let iter = ret.into_index_worktree_iter(Vec::new()).unwrap().into_iter();
        for item in iter {
            return Ok(true);
        }
        Ok(false)
    }
    /// 是否为推送提交
    ///
    pub fn unpushed_commits(&self) -> Result<bool, String> {
        let repo = &self.repository;
        let head_name = match repo.head_name().unwrap() {
            Some(name) => name.as_bstr().to_string().split('/').last().unwrap().to_string(),
            None => return Ok(false)    // Head分离的情况下，不存在分支信息
        };   // 获取当前分支名字
        let remote_names: Vec<String> = repo
            .remote_names()
            .into_iter()
            .map(|name| name.to_string())
            .collect();
        // 最近head指向分支最新的提交，
        let latest_commit_id = repo.head_commit().unwrap().id;

        // 在所有远程分支上找，是否能找到最新的本地提交，找得到的话，就说明已经提交过了，找不到就说明远程的不是最新的
        for remote_name in remote_names {
            let ref_string = format!("refs/remotes/{}/{}", remote_name, head_name);
            if let Ok(full_name) = refs::FullName::try_from(ref_string) {
                let remote_head = repo
                    .find_reference(&full_name) // Pass as FullNameRef reference
                    .unwrap()
                    .peel_to_commit()
                    .unwrap();
                let found = remote_head.ancestors().all().unwrap().find(|commit| commit.as_ref().unwrap().id == latest_commit_id);
                if !found.is_some() {
                    return Ok(true);
                }
            } else {
                return Err("Failed to convert ref_name to FullNameRef".to_string());
            }
        }
        Ok(false)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_dirty() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL");
        println!("test_is_dirty: {}", provider.is_dirty().unwrap());
    }

    #[test]
    fn test_untracked_files() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL");
        println!("test_untracked_files: {:?}", provider.untracked_files().unwrap());
    }

    #[test]
    fn test_has_modified_files() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL");
        println!("test_has_modified_files： {}", provider.has_modified_files().unwrap());
    }

    #[test]
    fn test_modified_files() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL");
        println!("test_modified_files： {:?}", provider.modified_files().unwrap());
    }

    #[test]
    fn test_uncommit() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL");
        println!("test_uncommit: {}", provider.uncommit().unwrap());
    }

    #[test]
    fn test_unpushed_commits() {
        let provider = GitDataProvider::new(r"E:\workSpace\Rust\GQL");
        println!("test_unpushed_commits: {}", provider.unpushed_commits().unwrap());
    }
}