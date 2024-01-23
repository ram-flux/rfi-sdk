use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Contact {
    pub id: i32,
    pub user_id: i32,
    pub friend_id: i32,
    pub friend_account: String,
    pub gender: String,
    pub name: String,
    pub avatar: String,
    pub bio: String,
    pub remark: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
