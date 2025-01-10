use crate::core::cache::GitCache;
use crate::core::handle;
use crate::utils::consts::GIT_CACHE;
use crate::utils::dirs;
use anyhow::Result;
use chrono::Local;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use tauri::Manager;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

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
