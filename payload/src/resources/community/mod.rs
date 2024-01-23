use chrono::prelude::*;

pub mod admin;
pub mod member;
pub mod post;
pub mod post_reply;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Community {
    pub id: u32,
    pub father_id: Option<u32>,
    pub user_id: u32,
    pub name: String,
    pub bio: String,
    pub passwd: Option<String>,
    pub announcement: Option<String>,
    pub pinned: bool,
    pub status: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
