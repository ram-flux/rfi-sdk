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
    pg_table_name = "member",
    sqlite_table_name = "member",
    primary_key = "id:u32",
    constraint = "im_member_id_idx"
)]
pub struct Member {
    pub r#type: u8,
    pub user_id: u32,
    pub name: String,
    pub avatar: String,
    pub sort: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Member {
    pub fn new(r#type: u8, user_id: u32, name: String, avatar: String, sort: i32) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            r#type,
            user_id,
            name,
            avatar,
            sort,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for Member {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
