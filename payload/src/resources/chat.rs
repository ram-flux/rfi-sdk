use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Chat {
    pub chat_id: i32,
    pub user_id: i32,
    pub r#type: String,
    pub msg_increase: i32,
    pub from_id: i32,
    pub from_public_key: String,
    pub from_name: String,
    pub from_avatar: String,
    pub from_unread_num: i32,
    pub from_msg_id: i32,
    pub from_msg: String,
    pub from_update: String,
    pub ext: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
