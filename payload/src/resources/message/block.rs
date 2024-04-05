//
//  Copyright 2024 Ram Flux, LLC.
//

use chrono::prelude::*;
use resource::{GenResourceID, Resource};
use sqlx::Sqlite;
#[derive(
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    Debug,
    resource::resource_macros::Resource,
    Default,
)]
#[resource(
    schema_name = "im",
    pg_table_name = "block",
    sqlite_table_name = "block",
    primary_key = "id:u32",
    constraint = "im_block_id_idx"
)]
pub struct Block {
    pub chat_id: i32,
    pub user_id: i32,
    pub blocks: Vec<u8>, // Assuming Block is another struct or a specific type
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Block {
    pub fn new(chat_id: i32, user_id: i32, blocks: Vec<u8>) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            chat_id,
            user_id,
            blocks,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for Block {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
