use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use rusqlite::Connection;
use tracing::{event, Level};

mod elements;
mod raw;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    // 获取第一个命令行参数
    let db_path = PathBuf::from({
        let mut args = std::env::args();
        args.next().unwrap();
        args.next().unwrap_or_else(|| {
            panic!(
                "{}",
                "db path not provided, usage: db path".red().to_string()
            )
        })
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

    let friends: Vec<u64> = {
        // 打开 friend.csv
        let raw = std::fs::read_to_string("friend.csv")?;
        // 逐行读取
        let mut friends = Vec::new();
        for line in raw.lines() {
            // 先分出第1~2个逗号之间的部分
            let line = line.split(',').nth(1).unwrap();
            // 转换成数字
            let uin = line.parse::<u64>().unwrap();
            friends.push(uin);
        }
        friends
    };

    let groups: Vec<u64> = {
        // 打开 group.csv
        let raw = std::fs::read_to_string("group.csv")?;
        // 逐行读取
        let mut groups = Vec::new();
        for line in raw.lines() {
            // 先分出第1个逗号之前的部分
            let line = line.split(',').next().unwrap();
            // 转换成数字
            let uin = line.parse::<u64>().unwrap();
            groups.push(uin);
        }
        groups
    };

    // 输出数据库中的所有的表
    // 获取所有的表名
    let tables = stmt.query_map([], |row| row.get::<_, String>(0))?;

    for table_name in tables {
        if table_name.is_err() {
            continue;
        }
        let table_name = table_name.unwrap();
        if !table_name.contains("$")
            && (table_name.contains("group") || table_name.contains("buddy"))
        {
            if table_name.contains("buddy") {
                // 检验是不是在好友列表里
                let uin = table_name
                    .split('_')
                    .last()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                if !friends.contains(&uin) {
                    // 警告
                    // event!(Level::WARN, "好友表 {} 不在好友列表里", table_name);
                }
            }
            if table_name.contains("group") {
                // 检验是不是在群列表里
                let uin = table_name
                    .split('_')
                    .last()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                if !groups.contains(&uin) {
                    // 警告
                    // 截出后6位, 然后 contains 检测
                    let uin = uin.to_string();
                    let uin = &uin[uin.len() - 6..];
                    let new_uin = uin.parse::<u64>().unwrap();
                    let finds = groups
                        .iter()
                        .filter(|&&x| x.to_string().contains(&new_uin.to_string()))
                        .collect::<Vec<_>>();
                    // if !finds.is_empty() {
                    //     println!("{table_name} {:?}", finds);
                    // }
                    event!(
                        Level::WARN,
                        "群表 {} 不在群列表里, 找到可能匹配的: {:?}",
                        table_name,
                        finds
                    );
                }
            }
            event!(Level::DEBUG, "找到表: {}", table_name);
            // 随便选一个出来
            let mut stmt = conn.prepare(&format!("SELECT * FROM {} limit 50", table_name))?;
            // let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table_name))?;
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
                // event!(Level::INFO, "找到数据: {}", data);
            }
        }
    }

    Ok(())
}

fn open_db(db_path: &PathBuf) -> Result<Connection> {
    Ok(Connection::open(db_path)?)
}
