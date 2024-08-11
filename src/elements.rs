use std::fmt::{Debug, Display};

pub type Uin = u32;

pub enum TextElement {
    RawText(String),
    At(u32),
}

impl TextElement {
    pub fn from_text(text: String) -> Self {
        TextElement::RawText(text)
    }

    pub fn from_at(at: Uin) -> Self {
        TextElement::At(at)
    }

    pub fn at_all() -> Self {
        TextElement::At(0)
    }

    pub fn is_at(&self) -> bool {
        matches!(self, TextElement::At(_))
    }

    pub fn is_text(&self) -> bool {
        matches!(self, TextElement::RawText(_))
    }

    pub fn is_at_all(&self) -> bool {
        matches!(self, TextElement::At(0))
    }

    pub fn at_from_raw_db(payload: &[u8]) -> anyhow::Result<Self> {
        if !payload.len() == 13 {
            return Err(anyhow::anyhow!("Invalid payload length"));
        }
        if payload[6] == 1 {
            return Ok(Self::at_all());
        }
        let uin = u32::from_be_bytes([payload[7], payload[8], payload[9], payload[10]]);
        Ok(Self::from_at(uin))
    }
}

impl Display for TextElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextElement::RawText(text) => write!(f, "{}", text),
            TextElement::At(uin) => {
                if self.is_at_all() {
                    write!(f, "@全体成员(0)")
                } else {
                    write!(f, "@{}", uin)
                }
            }
        }
    }
}

impl Debug for TextElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn at_all() {
        let text = TextElement::at_all();
        assert_eq!(text.to_string(), "@全体成员(0)");
    }

    #[test]
    fn from_raw_at() {
        //  [0, 1, 0, 0, 0, 9, 0, 0, 56, 101, 16, 0, 0] 3695888
        let payload = [0, 1, 0, 0, 0, 9, 0, 0, 56, 101, 16, 0, 0];
        let text = TextElement::at_from_raw_db(&payload).unwrap();
        assert_eq!(text.to_string(), "@3695888");
        // [0, 1, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0] 0
        let payload = [0, 1, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0];
        let text = TextElement::at_from_raw_db(&payload).unwrap();
        assert_eq!(text.to_string(), "@全体成员(0)");
    }
}
