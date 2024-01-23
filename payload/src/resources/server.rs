use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Server {
    pub id: i32,
    pub r#type: String,
    pub country: String,
    pub name: String,
    pub ip: String,
    pub port: i32,
    pub online_count: i32,
    pub ext: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
