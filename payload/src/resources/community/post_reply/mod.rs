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
    pg_table_name = "community_post_reply",
    sqlite_table_name = "community_post_reply",
    primary_key = "id:u32",
    constraint = "im_community_post_reply_id_idx"
)]
pub struct PostReply {
    pub community_id: u32,
    pub user_id: u32,
    pub post_id: u32,
    pub content: String,
    pub sort: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl PostReply {
    pub fn new(community_id: u32, user_id: u32, post_id: u32, content: String, sort: i32) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            community_id,
            user_id,
            post_id,
            content,
            sort,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for PostReply {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
