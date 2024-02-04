pub mod remark;
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
    sqlx::FromRow,
)]
#[resource(
    schema_name = "im",
    pg_table_name = "contact",
    sqlite_table_name = "contact",
    primary_key = "id:u32",
    constraint = "im_contact_id_idx"
)]
pub struct Contact {
    pub user_id: u32,
    pub friend_id: u32,
    pub friend_account: String,
    pub gender: String,
    pub name: String,
    pub avatar: String,
    pub bio: String,
    pub remark: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Contact {
    pub fn set_data(
        mut self,
        friend_account: String,
        gender: String,
        name: String,
        avatar: String,
        bio: String,
        remark: String,
    ) -> Self {
        self.friend_account = friend_account;
        self.gender = gender;
        self.name = name;
        self.avatar = avatar;
        self.bio = bio;
        self.remark = remark;
        self
    }

    pub fn new(user_id: u32, friend_id: u32) -> Self {
        let time = crate::utils::time::now();
        Self {
            user_id,
            friend_id,
            created_at: time,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for Contact {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
