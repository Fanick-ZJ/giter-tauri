use std::rc::Rc;

use gix::ObjectId;
use types::author::Author;
use anyhow::Result;

pub trait Cache {
    /// 获取commit的作者，返回值为一个`Vec<Author>`，每个元素为一个Author
    /// 第二个值为最后统计到的commit的id
    /// 返回错误会不从缓存中获取
    fn authors(&self, repo: &str) -> Result<(Vec<Author>, ObjectId)>;

    /// 设置commit的作者
    fn set_authors(&self, repo: &str, authors: &Vec<Author>, last_commit_id: &ObjectId);
}
