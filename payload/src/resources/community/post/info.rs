use chrono::prelude::*;
use resource::Resource;
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct PostInfo {
    pub name: String,
    pub content: String,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PostInfo {
    pub fn new(name: String, content: String) -> Self {
        Self {
            updated_at: Some(crate::utils::time::now()),
            name,
            content,
        }
    }
}

impl resource::GenResourceID for PostInfo {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}

impl Resource<sqlx::Sqlite> for PostInfo {
    type ResourceID = u32;

    async fn update<'c, E>(&self, id: &Self::ResourceID, executor: E) -> Result<(), resource::Error>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Sqlite>,
    {
        let sql = "UPDATE community_post SET 
        name = $1, content = $2,
        updated_at = $3
        WHERE id = $4;";
        sqlx::query(sql)
            .bind(&self.name)
            .bind(&self.content)
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
