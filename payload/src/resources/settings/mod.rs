//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod language;
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
    pg_table_name = "settings",
    sqlite_table_name = "settings",
    primary_key = "id:u32",
    constraint = "im_settings_id_idx"
)]
pub struct Settings {
    pub user_id: u32,
    pub language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Settings {
    pub fn new(user_id: u32, language: String) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            user_id,
            language,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for Settings {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
