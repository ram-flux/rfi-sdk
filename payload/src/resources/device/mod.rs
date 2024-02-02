use chrono::prelude::*;
use resource::{GenResourceID, Resource};
use sqlx::Sqlite;

pub mod token;
#[derive(
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    Debug,
    resource::resource_macros::Resource,
    Default,
    sqlx::FromRow,
)]
#[resource(
    schema_name = "im",
    pg_table_name = "device",
    sqlite_table_name = "device",
    primary_key = "device_id:u32",
    constraint = "im_device_id_idx"
)]
pub struct Device {
    // pub device_id: i64,
    pub public_key: String,
    pub server_public_key: String,
    pub server_private_key: String,
    //ios,android,win,mac,linux
    pub def: String,
    pub user_id: u32,
    pub token: String,
    pub proof: String,
    pub version: String,
    pub ext: Option<String>,
    pub last_ip: String,
    pub last_time: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl resource::GenResourceID for Device {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
