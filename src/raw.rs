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

impl RawData {
    pub fn new(
        time: i64,
        rand: i64,
        sender_uin: i64,
        msg_content: Vec<u8>,
        info: Vec<u8>,
    ) -> Self {
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
            let payload_len = u16::from_le_bytes(self.msg_content[ptr..ptr + 2].try_into().unwrap());
            ptr += 2;
            let payload = &self.msg_content[ptr..ptr + payload_len as usize];
            ptr += payload_len as usize;
            // print!("payload_type: {}, payload_len: {}, payload: {:?}", payload_type, payload_len, payload);
            match payload_type {
                0x01 => {
                    // UTF-16 的文本
                    let type_ = payload[0];
                    if type_ != 0x01 {
                        continue;
                    }
                    let text = String::from_utf16_lossy(
                        &payload[1..]
                            .chunks_exact(2)
                            .map(|x| u16::from_le_bytes(x.try_into().unwrap()))
                            .collect::<Vec<u16>>(),
                    );
                    print!("text: {}", text);
                },
                0x02 => {
                    // 表情
                    
                },
                _ => ()
            }
        }
        format!(
            "time: {}, rand: {}, color_code: {}, font: {}, effect: {}, char_set: {}, font_family: {} font_name: {}\n time: {}, rand: {}, sender_uin: {}",
            time, rand, color_code, font, effect, char_set, font_family, font_name, self.time, self.rand, self.sender_uin
        )
    }
}
