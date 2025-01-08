use gix::ObjectId;
use types::author::Author;

pub trait Cache {
    /// 获取commit的作者，返回值为一个`Vec<Author>`，每个元素为一个Author
    /// 第二个值为最后统计到的commit的id
    /// 返回错误会不从缓存中获取
    fn get_contributors(&self) -> Result<Vec<Author>, ObjectId>;
}
