pub mod reply;

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
    pg_table_name = "account_elf",
    sqlite_table_name = "account_elf",
    primary_key = "id:u32",
    constraint = "im_account_elf_id_idx"
)]
pub struct Apply {
    pub r#type: u8,
    pub type_id: u8,
    pub user_id: u32,
    pub content: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Apply {
    pub fn new(r#type: u8, type_id: u8, user_id: u32, content: String, status: String) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            r#type,
            type_id,
            user_id,
            content,
            status,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for Apply {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
