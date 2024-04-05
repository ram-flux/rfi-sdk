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
    pg_table_name = "community_member",
    sqlite_table_name = "community_member",
    primary_key = "user_id:u32, community_id:u32",
    constraint = "community_member_pkey"
)]
pub struct CommunityMember {
    pub id: u32,
    pub r#type: u8,
    // pub user_id: u32,
    // pub community_id: u32,
    pub name: String,
    pub avatar: String,
    pub sort: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl CommunityMember {
    pub fn new(member_id: u32, r#type: u8, name: String, avatar: String, sort: i32) -> Self {
        Self {
            id: member_id,
            created_at: crate::utils::time::now(),
            r#type,
            // user_id,
            // community_id,
            name,
            avatar,
            sort,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for CommunityMember {
    type Target = (u32, u32);
    // type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1)
            .map_err(|_| resource::Error::GenIdFailure)?;
        let id1 = id_worker
            .next_id()
            .map_err(|_| resource::Error::GenIdFailure)? as u32;
        let id2 = id_worker
            .next_id()
            .map_err(|_| resource::Error::GenIdFailure)? as u32;
        Ok((id1, id2))
        // Ok(id1)
    }
}
