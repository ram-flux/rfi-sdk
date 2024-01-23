use chrono::prelude::*;

pub mod avatar;
pub mod community;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Account {
    pub user_id: u32,
    pub public_key: String,
    pub account: String,
    pub gender: u8,
    pub name: String,
    pub avatar: String,
    pub bio: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
