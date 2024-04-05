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
    pg_table_name = "elf",
    sqlite_table_name = "elf",
    primary_key = "id:u32",
    constraint = "im_elf_id_idx"
)]
pub struct Elf {
    pub r#type: u8,
    pub token: String,
    pub name: String,
    pub avatar: String,
    pub ext: String,
    pub status: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Elf {
    pub fn new(
        r#type: u8,
        token: String,
        name: String,
        avatar: String,
        ext: String,
        status: u8,
    ) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            r#type,
            token,
            name,
            avatar,
            ext,
            status,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for Elf {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
