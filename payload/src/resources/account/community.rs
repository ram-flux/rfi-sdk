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
    pg_table_name = "account_community",
    sqlite_table_name = "account_community",
    primary_key = "id:u32",
    constraint = "im_account_community_id_idx"
)]
pub struct AccountCommunity {
    pub community_id: u32,
    pub user_id: u32,
    pub name: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl AccountCommunity {
    pub fn new(community_id: u32, user_id: u32, name: String, avatar: String) -> Self {
        Self {
            community_id,
            user_id,
            name,
            avatar,
            created_at: crate::utils::time::now(),
            updated_at: Some(crate::utils::time::now()),
        }
    }
}

impl resource::GenResourceID for AccountCommunity {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
