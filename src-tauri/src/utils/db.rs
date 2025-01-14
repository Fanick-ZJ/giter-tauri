use std::path::PathBuf;
use rusqlite::Connection;
use anyhow::Result;

pub fn conn_db(path: PathBuf) -> Result<Connection> {
  Ok(Connection::open(path)?)
}