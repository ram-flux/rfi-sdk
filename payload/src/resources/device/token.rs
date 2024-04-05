//
//  Copyright 2024 Ram Flux, LLC.
//

use chrono::{DateTime, Utc};
use resource::Resource;

#[derive(
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    Debug,
    // resource::resource_macros::Resource,
    Default,
    sqlx::FromRow,
)]
// #[resource(
//     schema_name = "public",
//     pg_table_name = "device",
//     sqlite_table_name = "device",
//     primary_key = "device_id:u32",
//     constraint = "im_device_id_idx"
// )]
pub struct Token {
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl resource::GenResourceID for Token {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}

impl Resource<sqlx::Sqlite> for Token {
    type ResourceID = u32;

    async fn update<'c, E>(&self, id: &Self::ResourceID, executor: E) -> Result<(), resource::Error>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Sqlite>,
    {
        let sql = "UPDATE device SET 
        token = $1, 
        updated_at = $2
        WHERE device_id = $3;";
        sqlx::query(sql)
            .bind(&self.token)
            .bind(self.updated_at)
            .bind(id)
            .execute(executor)
            .await?;
        Ok(())
    }

    async fn insert<'c, E>(
        &self,
        _id: &Option<Self::ResourceID>,
        _executor: E,
    ) -> Result<(), resource::Error>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Sqlite>,
    {
        todo!()
    }

    async fn upsert<'c, E>(
        &self,
        _id: &Option<Self::ResourceID>,
        _executor: E,
    ) -> Result<(), resource::Error>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Sqlite>,
    {
        todo!()
    }

    async fn drop<'c, E>(_id: &Self::ResourceID, _executor: E) -> Result<(), resource::Error>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Sqlite>,
    {
        todo!()
    }
}
