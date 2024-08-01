use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use rusqlite::Connection;
use tracing::{event, Level};

mod raw;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    // 获取第一个命令行参数
    let db_path = PathBuf::from({
        let mut args = std::env::args();
        args.next().unwrap();
        args.next()
            .unwrap_or_else(|| { panic!("{}", "db path not provided, usage: db path".red().to_string()) })
    });
    // 打印日志
    event!(Level::INFO, "正在转换 {:?} 数据库", db_path);
    // 校验一下路径
    if !db_path.exists() {
        event!(Level::ERROR, "数据库文件 {:?} 不存在", db_path);
        return Ok(());
    }
    if !db_path.is_file() {
        event!(Level::ERROR, "{:?} 不是一个文件", db_path);
        return Ok(());
    }
    // 打开数据库
    let conn = match open_db(&db_path) {
        Ok(conn) => conn,
        Err(e) => {
            event!(Level::ERROR, "打开数据库失败: {:?}", e);
            return Ok(());
        }
    };

    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;

    // 输出数据库中的所有的表
    // 获取所有的表名
    let tables = stmt.query_map([], |row| row.get::<_, String>(0))?;

    for table_name in tables {
        if table_name.is_err() {
            continue;
        }
        let table_name = table_name.unwrap();
        event!(Level::DEBUG, "找到表: {}", table_name);
        if !table_name.contains("$") {
            // 随便选一个出来
            let mut stmt = conn.prepare(&format!("SELECT * FROM {} LIMIT 3", table_name))?;
            let rows = stmt.query_map([], |row| {
                let time: i64 = row.get(0)?;
                let rand: i64 = row.get(1)?;
                let sender_uin: i64 = row.get(2)?;
                let msg_content: Vec<u8> = row.get(3)?;
                let info: Vec<u8> = row.get(4)?;
                Ok(raw::RawData::new(time, rand, sender_uin, msg_content, info))
            })?;
            for row in rows {
                if row.is_err() {
                    continue;
                }
                let row = row.unwrap();
                let data = row.decode();
                event!(Level::DEBUG, "找到数据: {}", data);
            }
        }
    }

    Ok(())
}

fn open_db(db_path: &PathBuf) -> Result<Connection> {
    Ok(Connection::open(db_path)?)
}
