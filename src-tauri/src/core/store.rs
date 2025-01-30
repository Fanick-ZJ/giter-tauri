use std::path::PathBuf;

use rusqlite::params;

use crate::{
    types::{
        cache::RepoPath,
        store::{self, Repository},
    },
    utils::{
        db::conn_db,
        dirs::{self, repo_default_alias},
    },
};

#[derive(Debug, Clone)]
pub struct GitStore {
    path: PathBuf,
}

impl GitStore {
    pub fn new() -> GitStore {
        GitStore {
            path: dirs::store_file().unwrap(),
        }
    }

    fn is_stored(&self, path: &str) -> bool {
        let sql = "select count(*) from repository where path=?1";
        let conn = conn_db(self.path.clone()).unwrap();
        let mut stmt = conn.prepare(sql).unwrap();
        let count: i32 = stmt.query_row([path], |row| row.get::<_, i32>(0)).unwrap();
        count > 0
    }

    pub fn insert_repo(
        &self,
        path: &str,
        alias: &str,
        has_watch: bool,
        order: i32,
        top: bool,
    ) -> Result<store::Repository, String> {
        let sql = "insert into repository (id, path, alias, has_watch, `order`, top) values (null,?1,?2,?3,?4,?5)";
        let conn = conn_db(self.path.clone()).unwrap();
        let stmt = conn.execute(sql, params![path, alias, has_watch, order, top]);
        match stmt {
            Ok(_) => {
                let last_id = conn.last_insert_rowid();
                log::info!("add repo success: {}", path);
                Ok(Repository::new(
                    last_id as i32,
                    path.to_string(),
                    alias.to_string(),
                    has_watch,
                    order,
                    top,
                ))
            }
            Err(e) => {
                log::error!("add repo error: {:?}", e);
                Err(e.to_string())
            }
        }
    }

    pub fn add_repo(
        &self,
        path: String,
        alias: Option<String>,
        has_watch: Option<bool>,
        order: Option<i32>,
        top: Option<bool>,
    ) -> Result<store::Repository, String> {
        // 查询是否存在;
        if self.is_stored(&path) {
            return Err("repo already exist".to_string());
        }
        let alias = alias.unwrap_or(repo_default_alias(&path));
        let has_watch = has_watch.unwrap_or(true);
        let order = order.unwrap_or(-1);
        let top = top.unwrap_or(false);
        // 插入
        self.insert_repo(&path, &alias, has_watch, order, top)
    }

    pub fn get_repos(&self) -> Result<Vec<store::Repository>, String> {
        let sql = "select * from repository";
        let conn = conn_db(self.path.clone()).unwrap();
        let mut stmt = conn.prepare(sql).unwrap();
        let repos = stmt
            .query_map([], |row| {
                Ok(Repository {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    alias: row.get(2)?,
                    has_watch: row.get(3)?,
                    order: row.get(4)?,
                    top: row.get(5)?,
                })
            })
            .unwrap();
        let mut result = Vec::new();
        for repo in repos {
            result.push(repo.unwrap());
        }
        Ok(result)
    }

    pub fn update_repo(&self, repo: store::Repository) -> Result<(), String> {
        let sql =
            "update repository set path=?1, alias=?2, has_watch=?3, `order`=?4, top=?5 where id=?6";
        let conn = conn_db(self.path.clone()).unwrap();
        let stmt = conn.execute(
            sql,
            params![
                repo.path,
                repo.alias,
                repo.has_watch,
                repo.order,
                repo.top,
                repo.id
            ],
        );
        match stmt {
            Ok(_) => {
                log::info!("update repo success: {}", repo.path);
                Ok(())
            }
            Err(e) => {
                log::error!("update repo error: {:?}", e);
                Err(e.to_string())
            }
        }
    }

    pub fn delete_repo(&self, path: &RepoPath) -> Result<(), String> {
        let sql = "delete from repository where path=?1";
        let conn = conn_db(self.path.clone()).unwrap();
        let stmt = conn.execute(sql, [path]);
        match stmt {
            Ok(_) => {
                log::info!("delete repo success: {}", path);
                Ok(())
            }
            Err(e) => {
                log::error!("delete repo error: {:?}", e);
                Err(e.to_string())
            }
        }
    }
}
