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
pub struct AccountElf {
    pub elf_id: u32,
    pub user_id: u32,
    pub name: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl AccountElf {
    pub fn new(elf_id: u32, user_id: u32, name: String, avatar: String) -> Self {
        Self {
            elf_id,
            user_id,
            name,
            avatar,
            created_at: crate::utils::time::now(),
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for AccountElf {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
