use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct PostReply {
    pub id: u32,
    pub community_id: u32,
    pub user_id: u32,
    pub post_id: u32,
    pub content: String,
    pub sort: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
