use serde_json::{self, Value};
use std::fs::File;

pub const GUILD_ID: u64 = 1031279445065089084;
pub const COMPLETE_CHANNEL_ID: u64 = 1078028132872880279;
pub const DALL_E_CHANNEL_ID: u64 = 1078027542335848518;
pub const EDIT_CHANNEL_ID: u64 = 1078028295712542861;
pub const BUG_REPORT_CHANNEL_ID: u64 = 1078040233242787983;
pub const MOD_ROLE_ID: u64 = 1077335879540027455;
pub const MEMBER_REPORT_CHANNEL_ID: u64 = 1078374161765912727;
pub const CONFESSION_CHANNEL_ID: u64 = 1078745380818276432;
pub const CHAT_COMPLETION_CHANNEL_ID: u64 = 1085642623471075341;
pub const LOG_CHANNEL_ID: u64 = 1077635970976981043;

#[derive(Debug)]
pub struct Error {
    pub io: Option<std::io::Error>,
    pub json: Option<serde_json::Error>,
}

pub fn load(path: &str) -> Result<Value, Error> {
    let file = match File::open(path) {
        Ok(v) => v,
        Err(e) => {
            return Err(Error {
                io: Some(e),
                json: None,
            })
        }
    };

    match serde_json::from_reader(file) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error {
            io: None,
            json: Some(e),
        }),
    }
}

pub trait Token {
    fn discord_token(&self) -> Option<String>;
    fn openai_key(&self) -> Option<String>;
}

impl Token for Value {
    fn discord_token(&self) -> Option<String> {
        Some(self.as_object()?.get("discord_token")?.as_str()?.to_string())
    }

    fn openai_key(&self) -> Option<String> {
        Some(self.as_object()?.get("openai_key")?.as_str()?.to_string())
    }
}
