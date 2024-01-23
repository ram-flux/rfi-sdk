use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Device {
    pub device_id: i64,
    pub public_key: String,
    pub server_public_key: String,
    pub server_private_key: String,
    //ios,android,win,mac,linux
    pub def: String,
    pub user_id: String,
    pub token: String,
    pub proof: String,
    pub version: String,
    pub ext: Option<String>,
    pub last_ip: String,
    pub last_time: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
