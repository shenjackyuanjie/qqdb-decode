use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use rusqlite::Connection;
use tracing::{event, Level};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    // 获取第一个命令行参数
    let db_path = PathBuf::from({
        let mut args = std::env::args();
        args.next().unwrap();
        args.next()
            .expect(&"db path not provided, usage: db path".red())
    });
    // 打印日志
    event!(Level::INFO, "正在转换 {:?} 数据库", db_path);
    // 校验一下路径
    if !db_path.exists() {
        event!(Level::ERROR, "数据库文件 {:?} 不存在", db_path);
        return;
    }
    if !db_path.is_file() {
        event!(Level::ERROR, "{:?} 不是一个文件", db_path);
        return;
    }
    // 打开数据库
    let conn = match open_db(&db_path) {
        Ok(conn) => conn,
        Err(e) => {
            event!(Level::ERROR, "打开数据库失败: {:?}", e);
            return;
        }
    };
}

fn open_db(db_path: &PathBuf) -> Result<Connection> {
    Ok(Connection::open(db_path)?)
}
