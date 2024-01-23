use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Broadcast {
    pub id: i32,
    pub token: String,
    pub name: String,
    pub avatar: String,
    pub ext: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
