use std::collections::HashMap;

use serde_json::Value;

use crate::util::deserialize_from_map;

use super::author::Author;

#[derive(Default, Debug)]
pub struct FilterConditions {
    pub last_id: Option<String>,
    pub start: usize,
    pub count: usize,
    pub author: Author,
    pub start_time: i64,
    pub end_time: i64,
}

impl FilterConditions {
   pub fn build_from_sv_map(map: &HashMap<String, Value>) -> Self {
        let mut conditions = FilterConditions::default();
        conditions.last_id = deserialize_from_map::<Option<String>>(map, "lastId", None);
        conditions.start = deserialize_from_map::<usize>(map, "start", 0);
        conditions.count = deserialize_from_map::<usize>(map, "count", i32::MAX as usize);
        conditions.author = deserialize_from_map::<Author>(map, "author", Author::default());
        conditions.start_time = deserialize_from_map::<i64>(map, "startTime", 0);
        conditions.end_time = deserialize_from_map::<i64>(map, "endTime", i64::MAX);
        conditions
   }
}