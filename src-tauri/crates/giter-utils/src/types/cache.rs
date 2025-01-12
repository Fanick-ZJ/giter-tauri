use gix::ObjectId;
use types::author::Author;
use crate::types::branch::Branch;

pub trait Cache {
    fn authors(&self, repo: &str) -> Option<Vec<Author>>;
    /// 获取commit的作者，返回值为一个`Vec<Author>`，每个元素为一个Author
    /// 第二个值为最后统计到的commit的id
    /// 返回错误会不从缓存中获取
    fn branch_authors(&self, repo: &str, branch: &Branch) -> Option<(Vec<Author>, ObjectId)>;

    /// 设置commit的作者
    fn set_authors(&mut self, repo: &str, authors: &Vec<Author>, branch: &Branch, last_commit_id: &ObjectId);

    fn clear(&mut self, repo: &str);

    fn clear_all(&mut self);
}
