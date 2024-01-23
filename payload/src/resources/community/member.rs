use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Member {
    pub id: u32,
    pub r#type: u8,
    pub user_id: u32,
    pub name: String,
    pub avatar: String,
    pub sort: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
