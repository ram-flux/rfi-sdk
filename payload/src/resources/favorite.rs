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
    pg_table_name = "favorite",
    sqlite_table_name = "favorite",
    primary_key = "id:u32",
    constraint = "im_favorite_id_idx"
)]
pub struct Favorite {
    pub user_id: u32,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Favorite {
    pub fn new(user_id: u32, content: String) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            user_id,
            content,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for Favorite {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
