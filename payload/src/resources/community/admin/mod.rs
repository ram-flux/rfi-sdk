//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod typ;

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
    pg_table_name = "community_admin",
    sqlite_table_name = "community_admin",
    primary_key = "id:u32",
    constraint = "im_community_admin_id_idx"
)]
pub struct CommunityAdmin {
    pub r#type: u8,
    pub community_id: u32,
    pub user_id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl CommunityAdmin {
    pub fn new(r#type: u8, community_id: u32, user_id: u32) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            r#type,
            community_id,
            user_id,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for CommunityAdmin {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
