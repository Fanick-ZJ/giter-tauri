use serde::{Deserialize, Serialize};

use crate::make_serializable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandError {
    pub message: String,
    pub func: String,
    // data 需要容纳函数的传入参数
    pub data: Option<Vec<String>>,
}