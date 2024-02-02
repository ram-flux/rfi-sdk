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
    pg_table_name = "nav",
    sqlite_table_name = "nav",
    primary_key = "id:u32",
    constraint = "im_nav_id_idx"
)]
pub struct Nav {
    pub r#type: String,
    pub type_id: u32,
    pub user_id: u32,
    pub sort: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Nav {
    pub fn new(r#type: String, type_id: u32, user_id: u32, sort: u32) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            r#type,
            type_id,
            user_id,
            sort,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for Nav {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
