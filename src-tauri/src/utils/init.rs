use crate::utils::dirs;
use anyhow::Result;
use chrono::Local;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use rusqlite::Connection;
use std::fs;

pub fn init_log() -> Result<()> {
    let log_dir = dirs::app_logs_dir()?;
    if !log_dir.exists() {
        let _ = fs::create_dir_all(&log_dir);
    }

    let log_level = LevelFilter::Info;

    let local_time = Local::now().format("%Y-%m-%d_%H-%M").to_string();
    let log_file = format!("{}.log", local_time);
    let log_file = log_dir.join(log_file);

    let log_pattern = "{d(%Y-%m-%d %H:%M)}";

    let encode = Box::new(PatternEncoder::new(log_pattern));

    // 控制台日志输出
    let stdout = ConsoleAppender::builder().encoder(encode.clone()).build();
    // 文件日志输出
    let tofile = FileAppender::builder()
        .encoder(encode.clone())
        .build(log_file)?;

    let mut logger_builder = Logger::builder();
    let root_builder = Root::builder();
    logger_builder = logger_builder.appender("file");

    let (config, _) = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(tofile)))
        .logger(logger_builder.additive(false).build("app", log_level))
        .build_lossy(root_builder.build(log_level));

    log4rs::init_config(config)?;

    Ok(())
}

pub fn init_cache() -> Result<()> {
    let database_path = dirs::database_dir()?;
    if !database_path.exists() {
        fs::create_dir_all(&database_path)?;
    }
    let cache_path = database_path.join("cache.db");
    let conn = Connection::open(cache_path)?;
    conn.execute(
        "create table if not exists branch_author (
        id integer primary key autoincrement,
        path text not null,
        branch text not null,
        authors text not null,
        last_commit_id varchar(20) not null
      )",
        (),
    )?;
    Ok(())
}

pub fn init_store() -> Result<()> {
    let database_path = dirs::database_dir()?;
    if !database_path.exists() {
        fs::create_dir_all(&database_path)?;
    }
    let store_path = database_path.join("store.db");
    println!("{:?}", store_path);
    let conn = Connection::open(store_path)?;
    conn.execute(
        "
    create table if not exists repository (
        id integer primary key autoincrement,
        path text not null unique,
        alias text default NULL,
        has_watch integer default 1,
        `order` integer default NULL,
        top integer default 0,
        trusted integer default 0,
        to_trusted integer default 0
    )
    ",
        (),
    )
    .unwrap_or_else(|e| {
        println!("{}", e);
        (1)
    });
    Ok(())
}

pub fn init_config() -> Result<()> {
    let database_path = dirs::database_dir()?;
    if !database_path.exists() {
        fs::create_dir_all(&database_path)?;
    }
    let config_path = database_path.join("config.db");
    println!("{:?}", config_path);
    let conn = Connection::open(config_path)?;
    conn.execute(
        "
    create table if not exists config (
        id integer primary key autoincrement,
        key varchar(255) not null unique,
        value text not null
    )
    ",
        (),
    )
    .unwrap_or_else(|e| {
        println!("{}", e);
        (1)
    });
    Ok(())
}
