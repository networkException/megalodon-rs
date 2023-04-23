use super::{Emoji, Field, Source};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub discoverable: Option<bool>,
    pub group: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub emojis: Vec<Emoji>,
    pub moved: Option<Box<Account>>,
    pub fields: Vec<Field>,
    pub bot: bool,
    pub source: Source,
}
