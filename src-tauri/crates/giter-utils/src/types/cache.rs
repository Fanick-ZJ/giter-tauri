use std::collections::HashMap;

use super::{
    author::Author, contribution::CommitStatistic, credential::Credential, file::FileHistoryEntry,
};
use crate::types::branch::Branch;
use git2::Oid;

pub trait Cache {
    fn authors(&self, repo: &str) -> Option<Vec<Author>>;
    /// 获取commit的作者，返回值为一个`Vec<Author>`，每个元素为一个Author
    /// 第二个值为最后统计到的commit的id
    /// 返回错误会不从缓存中获取
    fn branch_authors(&self, repo: &str, branch: &Branch) -> Option<(Vec<Author>, Oid)>;

    /// 获取分支的贡献者统计
    ///
    fn branch_contribution(
        &self,
        repo: &str,
        branch: &Branch,
    ) -> Option<(HashMap<String, CommitStatistic>, Oid)>;

    fn set_branch_contribution(
        &mut self,
        repo: &str,
        branch: &Branch,
        contrib: &HashMap<String, CommitStatistic>,
        last_commit_id: &Oid,
    );

    /// 设置commit的作者
    fn set_authors(
        &mut self,
        repo: &str,
        authors: &Vec<Author>,
        branch: &Branch,
        last_commit_id: &Oid,
    );

    // 清除单个仓库缓存
    fn clear(&mut self, repo: &str);
    /// 清除所有缓存
    fn clear_all(&mut self);

    // 获取和设置凭证
    fn get_credential(&self, repo: &str) -> Option<Credential>;
    fn set_credential(&mut self, repo: &str, credential: &Credential);

    // 获取文件历史缓存
    fn get_file_history(&self, repo: &str, file: &str) -> Option<Vec<FileHistoryEntry>>;
    fn set_file_history(&mut self, repo: &str, file: &str, history: &Vec<FileHistoryEntry>);

    // 获取分支提交次数缓存
    fn get_reference_commit_count(&self, repo: &str, oid: &str) -> Option<(i32, String)>;
    fn set_reference_commit_count(&self, repo: &str, oid: &str, last_id: &str, count: i64);
}
