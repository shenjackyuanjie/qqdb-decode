use colored::Colorize;
use tracing::{event, Level};

/*
字段 Time 类型是 INTEGER
字段 Rand 类型是 INTEGER
字段 SenderUin 类型是 INTEGER
字段 MsgContent 类型是 BLOB
字段 Info 类型是 BLOB
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawData {
    /// Time: INTEGER
    pub time: i64,
    /// Rand: INTEGER
    pub rand: i64,
    /// SenderUin: INTEGER
    pub sender_uin: i64,
    /// MsgContent: BLOB
    pub msg_content: Vec<u8>,
    /// Info: BLOB
    pub info: Vec<u8>,
}

const TEXT_TYPE0: [u8; 47] = [
    84, 68, 1, 1, 1, 0, 1, 30, 0, 131, 225, 181, 225, 132, 225, 153, 225, 149, 225, 175, 225, 132,
    225, 132, 225, 133, 225, 168, 225, 143, 225, 133, 225, 132, 225, 143, 225, 149, 225, 4, 0, 0,
    0, 1, 0, 0, 0,
];
const TEXT_TYPE8: [u8; 8] = [0, 1, 0, 4, 82, 204, 245, 208];

impl RawData {
    pub fn new(time: i64, rand: i64, sender_uin: i64, msg_content: Vec<u8>, info: Vec<u8>) -> Self {
        Self {
            time,
            rand,
            sender_uin,
            msg_content,
            info,
        }
    }

    pub fn decode(&self) -> String {
        let mut ptr = 8; // 跳过 前 8 字节
        let time = u32::from_le_bytes(self.msg_content[ptr..ptr + 4].try_into().unwrap());
        ptr += 4;
        let rand = u32::from_be_bytes(self.msg_content[ptr..ptr + 4].try_into().unwrap());
        ptr += 4;
        let mut color_code_data: [u8; 4] = self.msg_content[ptr..ptr + 4].try_into().unwrap();
        color_code_data[0] = 0;
        let color_code = u32::from_be_bytes(color_code_data);
        ptr += 4;
        let font: u8 = self.msg_content[ptr];
        ptr += 1;
        let effect: u8 = self.msg_content[ptr];
        ptr += 1;
        let char_set: u8 = self.msg_content[ptr];
        ptr += 1;
        let font_family: u8 = self.msg_content[ptr];
        ptr += 1;
        let name_len = u16::from_le_bytes(self.msg_content[ptr..ptr + 2].try_into().unwrap());
        ptr += 2;
        // font: utf16
        let font_name: String = String::from_utf16_lossy(
            &self.msg_content[ptr..ptr + name_len as usize]
                .chunks_exact(2)
                .map(|x| u16::from_le_bytes(x.try_into().unwrap()))
                .collect::<Vec<u16>>(),
        );
        ptr += name_len as usize;
        ptr += 2;
        while ptr < self.msg_content.len() {
            let payload_type: u8 = self.msg_content[ptr];
            ptr += 1;
            let payload_len =
                u16::from_le_bytes(self.msg_content[ptr..ptr + 2].try_into().unwrap());
            ptr += 2;
            let payload = &self.msg_content[ptr..ptr + payload_len as usize];
            ptr += payload_len as usize;
            // print!("payload_type: {}, payload_len: {}, payload: {:?}", payload_type, payload_len, payload);
            let type_ = if !payload.is_empty() {
                payload[0]
            } else {
                0x00
            };
            /*
            MsgText = 1
            MsgFace = 2
            MsgGroupImage = 3
            MsgPrivateImage = 6
            MsgVoice = 7
            MsgNickName = 18
            MsgVideo = 26 */
            match payload_type {
                0x01 => {
                    // UTF-16 的文本
                    // 理论上是文本
                    let mut inner_ptr = 0;
                    loop {
                        if inner_ptr >= payload.len() {
                            break;
                        }
                        let inner_type = payload[inner_ptr];
                        inner_ptr += 1;
                        let len = u16::from_le_bytes([payload[inner_ptr], payload[inner_ptr + 1]]);
                        inner_ptr += 2;
                        match inner_type {
                            0x00 => {
                                // 这是啥?
                                // 比对一下和 const 的区别，不一样再 print
                                // 可忽略
                                if payload[inner_ptr..inner_ptr + len as usize] != TEXT_TYPE0 {
                                    println!(
                                        "{}",
                                        format!(
                                            "type: {}, len: {}, payload_len: {} raw: {:?}",
                                            inner_type,
                                            len,
                                            payload_len,
                                            &payload[inner_ptr..inner_ptr + len as usize]
                                        )
                                        .red()
                                    );
                                }
                            }
                            _ => {

                            }
                            0x01 => {
                                let text = String::from_utf16_lossy(
                                    &payload[inner_ptr..inner_ptr + len as usize]
                                        .chunks_exact(2)
                                        .map(|x: &[u8]| u16::from_le_bytes(x.try_into().unwrap()))
                                        .collect::<Vec<u16>>(),
                                );
                                // println!("{}", format!("text: {}", text).green());
                            }
                            0x02 => {
                                // 网址后面的第一个
                                // println!("{len} {}", format!("url: {:?}", &payload[inner_ptr..inner_ptr + len as usize]).blue());
                                // 试试 utf8?
                                let text = String::from_utf8_lossy(
                                    &payload[inner_ptr..inner_ptr + len as usize],
                                );
                                // println!("{len} {}", format!("url: {}", text).blue());
                            }
                            0x03 => {
                                // 网址后面的第二个
                                // 字符串?
                                // platform (似乎是常量)
                                // 可忽略
                                let text = String::from_utf16_lossy(
                                    &payload[inner_ptr..inner_ptr + len as usize]
                                        .chunks_exact(2)
                                        .map(|x: &[u8]| u16::from_le_bytes(x.try_into().unwrap()))
                                        .collect::<Vec<u16>>(),
                                );
                                // println!("{}", format!("url: {}", text).purple());
                            }
                            0x06 => {
                                // 一个 @ 后面的东西
                                println!(
                                    "{}",
                                    format!("@: {:?}", &payload[inner_ptr..inner_ptr + len as usize])
                                        .yellow()
                                )
                            }
                            0x08 => {
                                // 这又是啥
                                if payload[inner_ptr..inner_ptr + len as usize] != TEXT_TYPE8 {
                                    println!(
                                        "{}",
                                        format!(
                                            "type: {}, len: {}, payload_len: {} raw: {:?}",
                                            inner_type,
                                            len,
                                            payload_len,
                                            &payload[inner_ptr..inner_ptr + len as usize]
                                        )
                                        .red()
                                    );
                                }
                            }
                            _ => println!(
                                "{}",
                                format!(
                                    "type: {}, len: {}, payload_len: {} raw: {:?}",
                                    inner_type,
                                    len,
                                    payload_len,
                                    &payload[inner_ptr..inner_ptr + len as usize]
                                )
                                .red()
                            ),
                        }
                        inner_ptr += len as usize;
                    }
                }
                0x02 => {
                    // 表情
                    if type_ != 0x01 {
                        continue;
                    }
                    let len = u16::from_le_bytes(payload[1..3].try_into().unwrap());
                    let mut id: u64 = 0;
                    for byte in payload[3..3 + len as usize].iter() {
                        id = (id << 8) | *byte as u64;
                    }
                    // println!("{}", format!("表情: {}", id).green());
                }
                0x03 => {
                    // 群图片
                    if type_ != 0x01 {
                        continue;
                    }
                }
                0x06 => {
                    // 私聊图片
                    if type_ != 0x01 {
                        continue;
                    }
                }
                0x07 => {
                    // 语音
                    if type_ != 0x01 {
                        continue;
                    }
                }
                0x12 => {
                    // 群名片
                    if type_ != 0x01 {
                        continue;
                    }
                }
                0x1a => {
                    // 视频
                    if type_ != 0x01 {
                        continue;
                    }
                }
                _ => (),
            }
        }
        format!(
            "time: {}, rand: {}, color_code: {}, font: {}, effect: {}, char_set: {}, font_family: {} font_name: {} time: {}, rand: {}, sender_uin: {}",
            time, rand, color_code, font, effect, char_set, font_family, font_name, self.time, self.rand, self.sender_uin
        )
    }
}
