use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

pub fn conn_db(path: PathBuf) -> Result<Connection> {
    Ok(Connection::open(path)?)
}
