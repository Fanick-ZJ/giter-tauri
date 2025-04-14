use std::collections::HashMap;

use serde_json::Value;

use crate::util::deserialize_from_map;

use super::author::Author;

#[derive(Default, Debug)]
pub struct FilterConditions {
    pub last_id: Option<String>,
    pub offset: Option<usize>,
    pub count: Option<usize>,
    pub author: Option<Author>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

impl FilterConditions {
   pub fn build_from_sv_map(map: &HashMap<String, Value>) -> Self {
        let mut conditions = FilterConditions::default();
        conditions.last_id = deserialize_from_map::<Option<String>>(map, "lastId", None);
        conditions.offset = deserialize_from_map::<Option<usize>>(map, "offset", None);
        conditions.count = deserialize_from_map::<Option<usize>>(map, "count", None);
        conditions.author = deserialize_from_map::<Option<Author>>(map, "author", None);
        conditions.start_time = deserialize_from_map::<Option<i64>>(map, "startTime", None);
        conditions.end_time = deserialize_from_map::<Option<i64>>(map, "endTime", None);
        conditions
   }
}