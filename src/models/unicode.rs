use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UnicodeChar {
    pub codepoint: String,
    pub name: String,
    pub block: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnicodeBlock {
    pub name: String,
    pub range_start: String,
    pub range_end: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnicodeResult {
    pub char: String,
    pub codepoint: String,
    pub name: String,
    pub utf8: String,
    pub utf16: String,
    pub utf16_le: String,
    pub block: String,
}
