//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod info;
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
    pg_table_name = "community_post",
    sqlite_table_name = "community_post",
    primary_key = "id:u32",
    constraint = "im_community_post_id_idx"
)]
pub struct Post {
    pub community_id: u32,
    pub user_id: u32,
    pub name: String,
    pub content: String,
    pub sort_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Post {
    pub fn new(
        community_id: u32,
        user_id: u32,
        name: String,
        content: String,
        sort_count: i32,
    ) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            community_id,
            user_id,
            name,
            content,
            sort_count,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for Post {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
