use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Session {
    pub id: i32,
    pub device_id: i32,
    pub user_id: i32,
    pub server_id: i32,
    pub server_ip: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
