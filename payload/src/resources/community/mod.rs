use chrono::prelude::*;

use resource::{GenResourceID, Resource};
use sqlx::Sqlite;

pub mod admin;
pub mod info;
pub mod member;
pub mod post;
pub mod post_reply;

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
    pg_table_name = "community",
    sqlite_table_name = "community",
    primary_key = "id:u32",
    constraint = "im_community_id_idx"
)]
pub struct Community {
    pub father_id: Option<u32>,
    pub user_id: u32,
    pub name: String,
    pub bio: String,
    pub passwd: Option<String>,
    pub announcement: Option<String>,
    pub pinned: bool,
    pub status: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Community {
    pub fn new(
        father_id: Option<u32>,
        user_id: u32,
        name: String,
        bio: String,
        passwd: Option<String>,
        announcement: Option<String>,
        pinned: bool,
        status: u8,
    ) -> Self {
        Self {
            father_id,
            user_id,
            name,
            bio,
            passwd,
            announcement,
            pinned,
            status,
            created_at: crate::utils::time::now(),
            updated_at: Some(crate::utils::time::now()),
        }
    }
}

impl resource::GenResourceID for Community {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
