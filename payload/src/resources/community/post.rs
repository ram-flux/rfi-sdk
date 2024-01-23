use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Post {
    pub id: u32,
    pub community_id: u32,
    pub user_id: u32,
    pub name: String,
    pub content: String,
    pub sort_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
