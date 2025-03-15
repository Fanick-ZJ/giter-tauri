use crate::{
    types::cache::{AuthorCache, BranchAuthorCache, BranchName, RepoPath},
    utils::{
        db::conn_db,
        dirs::cache_file,
    },
};
use git2::Oid;
use giter_utils::types::{author::Author, branch::Branch, cache::Cache as ProviderCache, contribution::CommitStatistic};
use rusqlite::Connection;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf
};

#[derive(Debug, Clone)]
pub struct GitCache {
    path: PathBuf,
}

impl GitCache {
    pub fn new() -> Self {
        let path = cache_file().unwrap();
        GitCache { path }
    }
}

impl GitCache {

    fn conn(&self) -> Connection {
        conn_db(self.path.clone()).unwrap()
    }

    /// 从数据库中获取缓存
    pub fn authors_cache(&self, repo: &str) -> BranchAuthorCache {
        let conn = conn_db(self.path.clone()).unwrap();
        let mut stmt = conn
            .prepare("select * from branch_author where path=?1")
            .unwrap();
        let caches = stmt.query_map([repo], |row| {
            match (
                row.get::<_, String>(2),
                row.get::<_, String>(3),
                row.get::<_, String>(4),
            ) {
                (Ok(branch), Ok(authors), Ok(last_commit_id)) => {
                    let authors: Vec<Author> = serde_json::from_str(&authors).unwrap();
                    if let Err(e) = Oid::from_str(&last_commit_id) {
                        log::error!(target: "app", "Failed to the app home directory: {}", e);
                        return Err(rusqlite::Error::QueryReturnedNoRows);
                    } else {
                        Ok((
                            branch,
                            AuthorCache {
                                authors: Some(authors),
                                last_commit_id: Some(last_commit_id),
                            },
                        ))
                    }
                }
                _ => Err(rusqlite::Error::QueryReturnedNoRows),
            }
        });
        if let Err(e) = caches {
            log::error!(target: "app", "Failed to the app home directory: {}", e);
            return HashMap::new();
        }
        let cache_iter = caches.unwrap();
        let mut cache_map: HashMap<BranchName, AuthorCache> = HashMap::new();
        for item in cache_iter {
            if let Ok((branch, authors)) = item {
                cache_map.insert(branch.to_string(), authors);
            }
        }
        cache_map
    }

    /// 更新作者缓存
    fn update_author_inner(&self, repo: RepoPath, author_cache: &BranchAuthorCache) {
        let insert_sql = "Insert into branch_author (id, path, branch, authors, last_commit_id) values (null, ?1, ?2, ?3, ?4)";
        let select_sql = "select count(*) from branch_author where path=?1 and branch=?2";
        let update_sql =
            "update branch_author set authors=?1, last_commit_id=?2 where path=?3 and branch=?4";
        // 查询是否存在
        let conn = conn_db(self.path.clone()).unwrap();
        let mut stmt = conn.prepare(select_sql).unwrap();
        for (branch, cache) in author_cache {
            let select: Result<i32, rusqlite::Error> =
                stmt.query_row([repo.as_str(), branch.as_str()], |row| row.get::<_, i32>(0));
            if let Err(e) = select {
                log::error!(target: "app", "Failed to the app home directory: {}", e);
                continue;
            }
            // 存在则更新
            if select.unwrap() > 0 {
                let update = conn.execute(
                    update_sql,
                    [
                        serde_json::to_string(&cache.authors).unwrap(),
                        cache.last_commit_id.clone().unwrap(),
                        repo.to_string(),
                        branch.to_string(),
                    ],
                );
                if let Err(e) = update {
                    log::error!(target: "app", "Failed to the app home directory: {}", e)
                }
            } else {
                // 不存在则插入
                let insert = conn.execute(
                    insert_sql,
                    [
                        repo.as_str(),
                        branch.as_str(),
                        serde_json::to_string(&cache.authors).unwrap().as_str(),
                        cache.last_commit_id.clone().unwrap().as_str(),
                    ],
                );
                if let Err(e) = insert {
                    log::error!(target: "app", "Failed to the app home directory: {}", e)
                }
            }
        }
    }

    pub fn update_author(&self, repo: RepoPath, author_cache: &BranchAuthorCache) {
        self.update_author_inner(repo, author_cache);
    }

    /// 清除所有作者缓存
    pub fn clear_author_cache(&self) {
        let conn = conn_db(self.path.clone()).unwrap();
        let clear_sql = "delete from branch_author where";
        let clear = conn.execute(clear_sql, []);
        match clear {
            Ok(count) => {
                log::info!("clear author cache success: {}", count)
            }
            Err(e) => {
                log::error!("clear author cache error: {:?}", e);
            }
        }
    }

    /// 清除指定仓库的作者缓存
    pub fn clear_repo_author_cache(&self, repo: RepoPath) {
        let conn = conn_db(self.path.clone()).unwrap();
        let clear_sql = "delete from branch_author where path=?1";
        let clear = conn.execute(clear_sql, [repo]);
        match clear {
            Ok(count) => {
                log::info!("clear author cache success: {}", count);
            }
            Err(e) => {
                log::error!("clear author cache error: {:?}", e);
            }
        }
    }

    /// 清除所有缓存
    pub fn clear_inner(&self) {
        self.clear_author_cache();
    }
}

impl ProviderCache for GitCache {
    fn branch_authors(&self, repo: &str, branch: &Branch) -> Option<(Vec<Author>, Oid)> {
        let conn = conn_db(self.path.clone()).unwrap();
        let mut stmt = conn
            .prepare("select * from branch_author where path=?1 and branch=?2")
            .unwrap();
        let caches = stmt.query_row([repo, branch.name.as_str()], |row| {
            match (row.get::<_, String>(3), row.get::<_, String>(4)) {
                (Ok(authors), Ok(last_commit_id)) => {
                    let authors: Vec<Author> = serde_json::from_str(&authors).unwrap();
                    let last_commit_id = Oid::from_str(&last_commit_id).unwrap();
                    Ok((authors, last_commit_id))
                }
                _ => Err(rusqlite::Error::QueryReturnedNoRows),
            }
        });
        if let Err(e) = caches {
            log::error!(target: "app", "Failed to the app home directory: {}", e);
            return None;
        }
        Some(caches.unwrap())
    }

    fn set_authors(
        &mut self,
        repo: &str,
        authors: &Vec<Author>,
        branch: &Branch,
        last_commit_id: &Oid,
    ) {
        let author_cache = AuthorCache {
            authors: Some(authors.clone()),
            last_commit_id: Some(last_commit_id.to_string()),
        };
        let map = HashMap::from([(branch.name.clone(), author_cache)]);
        self.update_author(repo.to_string(), &map);
    }

    fn clear(&mut self, repo: &str) {
        self.clear_repo_author_cache(repo.to_string());
    }

    fn clear_all(&mut self) {
        self.clear_inner();
    }

    fn authors(&self, repo: &str) -> Option<Vec<Author>> {
        let sql = "select * from branch_author where path=?1";
        let mut author_set: HashSet<Author> = HashSet::new();
        let conn = self.conn();
        let mut stmt = conn.prepare(sql).unwrap();
        let caches = stmt.query_map([repo], |row| match row.get::<_, String>(3) {
            Ok(authors) => {
                let authors: Vec<Author> = serde_json::from_str(&authors).unwrap();
                Ok(authors)
            }
            _ => Err(rusqlite::Error::QueryReturnedNoRows),
        });
        if let Err(e) = caches {
            println!("get cache error: {:?}", e);
            return None;
        }
        let cache_iter = caches.unwrap();
        for item in cache_iter {
            if let Ok(authors) = item {
                for author in authors {
                    author_set.insert(author);
                }
            }
        }
        Some(author_set.into_iter().collect())
    }
    
    fn branch_contribution(&self, repo: &str, branch: &Branch) -> Option<(HashMap<String, CommitStatistic>, Oid)> {
        let sql = "select contributors, last_commit_id from contribution where path=?1 and branch=?2 limit 1";
        let conn = self.conn();
        let mut stmt = conn.prepare(sql).unwrap();
        let caches = stmt.query_map([repo, &branch.name], |row| {
            match (row.get::<_, String>(0), row.get::<_, String>(1)) {
                (Ok(contrib), Ok(oid)) => {
                    let contrib: HashMap<String, CommitStatistic> = serde_json::from_str(&contrib).unwrap();
                    let oid = Oid::from_str(&oid);
                    if let Err(e) = oid {
                        println!("get cache error: {:?}", e);
                        return Err(rusqlite::Error::QueryReturnedNoRows);
                    }
                    let oid = oid.unwrap();
                    Ok((contrib, oid))
                },
                _ => Err(rusqlite::Error::QueryReturnedNoRows),
            }
        });
        if let Err(e) = caches {
            log::error!("get cache error: {:?}", e);
            return None;
        }
        let mut cache_iter = caches.unwrap();
        let res = cache_iter.next();
        match res {
            Some(Ok(contrib)) => Some(contrib),
            _ => None,
        }
    }
    
    fn set_branch_contribution(
        &mut self,
        repo: &str,
        branch: &Branch,
        contrib: &HashMap<String, CommitStatistic>,
        last_commit_id: &Oid,
    ) {
        let select_sql = "select count(*) from contribution where path=?1 and branch=?2"; 
        let insert_sql = "insert into contribution (id, path, branch, contributors, last_commit_id) values (null,?1,?2,?3,?4)";
        let update_sql = "update contribution set contributors=?1, last_commit_id=?2 where path=?3 and branch=?4";
        let conn = self.conn();
        let mut stmt = conn.prepare(select_sql).unwrap();
        let res = stmt.query_row([repo, &branch.name], |row| row.get::<_, i32>(0));
        if let Err(e) = res {
            log::error!("get cache error: {:?}", e);
            return;
        }
        let count = res.unwrap();
        if count > 0 {
            let update = conn.execute(update_sql, [
                serde_json::to_string(contrib).unwrap(),
                last_commit_id.to_string(),
                repo.to_string(),
                branch.name.clone(),
            ]);
            if let Err(e) = update {
                log::error!("update cache error: {:?}", e);
            }
        } else {
            let insert = conn.execute(insert_sql, [
                repo.to_string(),
                branch.name.clone(),
                serde_json::to_string(contrib).unwrap(),
                last_commit_id.to_string(),
            ]);
            if let Err(e) = insert {
                log::error!("insert cache error: {:?}", e);
            }
        }
    }
    
    fn get_credential(&self, host: &str) -> Option<giter_utils::types::credential::Credential> {
        let sql = "select username, password from credentials where host=?1";
        let conn = self.conn();
        let mut stmt = conn.prepare(sql).unwrap();
        let res = stmt.query_row([host], |row| {
            match (row.get::<_, String>(0), row.get::<_, String>(1)) {
                (Ok(username), Ok(password)) => {
                    Ok(giter_utils::types::credential::Credential::UsernamePassword(username, password))
                },
                _ => Err(rusqlite::Error::QueryReturnedNoRows),
            }
        });
        if let Err(e) = res {
            log::error!("get cache error: {:?}", e);
            return None; 
        }
        Some(res.unwrap())
    }
    
    fn set_credential(&mut self, host: &str, credential: &giter_utils::types::credential::Credential) {
        println!("set credential: {:?}", credential);
        // 提取 SQL 语句为常量
        const SELECT_SQL: &str = "SELECT COUNT(*) FROM credentials WHERE host=?1";
        const INSERT_SQL: &str = "INSERT INTO credentials (host, token, username, password) VALUES (?1, ?2, ?3, ?4)";
        const UPDATE_SQL: &str = "UPDATE credentials SET username=?1, password=?2 WHERE host=?3";
        
        let mut conn = self.conn();
        
        // 使用事务保证原子性
        let tx = match conn.transaction() {
            Ok(t) => t,
            Err(e) => {
                log::error!("开启事务失败: {:?}", e);
                return;
            }
        };

        // 统一错误处理逻辑
        let count: i32 = tx.query_row(SELECT_SQL, [host], |row| row.get(0))
            .unwrap_or_else(|e| {
                log::error!("查询凭证失败: {:?}", e);
                0
            });

        let result = match credential {
            giter_utils::types::credential::Credential::UsernamePassword(username, password) => {
                if count > 0 {
                    tx.execute(UPDATE_SQL, [username, "", password, host])
                } else {
                    println!("{}", INSERT_SQL);
                    tx.execute(INSERT_SQL, (host, "", username, password))
                }
            }
            giter_utils::types::credential::Credential::Token(token) => todo!()
        };
        println!("{:?}", result);
        // 统一处理执行结果
        match result {
            Ok(_) => {
                tx.commit().unwrap_or_else(|e| {
                    log::error!("提交事务失败: {:?}", e);
                });
                println!("{}", host);
                log::info!("凭证已更新: {}", host);
            }
            Err(e) => {
                log::error!("操作失败: {:?}", e);
                tx.rollback().unwrap_or_else(|e| {
                    log::error!("回滚事务失败: {:?}", e);
                });
            }
        }
    }  // 移除 todo!()
}
