// use chrono::prelude::*;

pub mod avatar;
pub mod community;
pub mod elf;

use resource::{GenResourceID, Resource};
use sqlx::Sqlite;
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
    pg_table_name = "account",
    sqlite_table_name = "account",
    primary_key = "id:u32",
    constraint = "im_account_id_idx"
)]
pub struct Account {
    pub user_id: u32,
    pub public_key: String,
    pub account: String,
    pub salt: String,
    pub gender: u8,
    pub name: String,
    pub avatar: String,
    pub bio: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Account {
    pub fn new(
        user_id: u32,
        public_key: String,
        account: String,
        salt: String,
        gender: u8,
        name: String,
        avatar: String,
        bio: String,
    ) -> Self {
        Self {
            user_id,
            public_key,
            account,
            salt,
            gender,
            name,
            avatar,
            bio,
            created_at: crate::utils::time::now(),
            updated_at: Some(crate::utils::time::now()),
        }
    }
}

impl resource::GenResourceID for Account {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
