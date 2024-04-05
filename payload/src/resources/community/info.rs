//
//  Copyright 2024 Ram Flux, LLC.
//

use chrono::prelude::*;

use resource::Resource;
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct CommunityInfo {
    pub name: String,
    pub bio: String,
    pub passwd: Option<String>,
    pub announcement: Option<String>,
    pub avatar: String,
    pub pinned: bool,
    pub status: u8,
    pub updated_at: Option<DateTime<Utc>>,
}

impl CommunityInfo {
    pub fn new(
        name: String,
        bio: String,
        passwd: Option<String>,
        announcement: Option<String>,
        avatar: String,
        pinned: bool,
        status: u8,
    ) -> Self {
        let time = crate::utils::time::now();
        Self {
            updated_at: Some(time),
            name,
            bio,
            passwd,
            announcement,
            pinned,
            status,
            avatar,
        }
    }
}

impl resource::GenResourceID for CommunityInfo {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}

impl Resource<sqlx::Sqlite> for CommunityInfo {
    type ResourceID = u32;

    async fn update<'c, E>(&self, id: &Self::ResourceID, executor: E) -> Result<(), resource::Error>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Sqlite>,
    {
        let sql = "UPDATE community SET 
        name = $1, bio = $2, passwd = $3, announcement = $4, pinned = $5, status = $6, avatar = $7,
        updated_at = $8
        WHERE id = $9;";
        sqlx::query(sql)
            .bind(&self.name)
            .bind(&self.bio)
            .bind(&self.passwd)
            .bind(&self.announcement)
            .bind(self.pinned)
            .bind(self.status)
            .bind(&self.avatar)
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
