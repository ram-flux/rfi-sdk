use chrono::prelude::*;

use resource::{resource_macros, GenResourceID, Resource};
use sqlx::Sqlite;
#[derive(
    serde::Deserialize, serde::Serialize, PartialEq, Debug, resource_macros::Resource, Default,
)]
#[resource(
    schema_name = "public",
    pg_table_name = "account",
    sqlite_table_name = "account",
    primary_key = "id:u32",
    constraint = "public_account_pkey"
)]
pub struct Avatar {
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Avatar {
    pub fn new(avatar: String) -> Self {
        Self {
            avatar,
            created_at: crate::utils::time::now(),
            updated_at: Some(crate::utils::time::now()),
        }
    }
}

impl resource::GenResourceID for Avatar {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
