use chrono::{DateTime, Utc};
use resource::Resource;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default, sqlx::FromRow)]
pub struct ContactRemark {
    pub remark: String,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ContactRemark {
    pub fn new(remark: String) -> Self {
        Self {
            remark,
            updated_at: Some(crate::utils::time::now()),
        }
    }
}

impl resource::GenResourceID for ContactRemark {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}

impl Resource<sqlx::Sqlite> for ContactRemark {
    type ResourceID = u32;

    async fn update<'c, E>(&self, id: &Self::ResourceID, executor: E) -> Result<(), resource::Error>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Sqlite>,
    {
        let sql = "UPDATE contact SET 
        remark = $1, 
        updated_at = $2
        WHERE id = $3;";
        sqlx::query(sql)
            .bind(&self.remark)
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
