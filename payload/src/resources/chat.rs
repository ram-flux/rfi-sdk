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
    pg_table_name = "chat",
    sqlite_table_name = "chat",
    primary_key = "id:u32",
    constraint = "im_chat_id_idx"
)]
pub struct Chat {
    pub user_id: i32,
    pub r#type: String,
    pub msg_increase: i32,
    pub from_id: i32,
    pub from_public_key: String,
    pub from_name: String,
    pub from_avatar: String,
    pub from_unread_num: i32,
    pub from_msg_id: i32,
    pub from_msg: String,
    pub from_update: String,
    pub ext: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Chat {
    pub fn new(
        user_id: i32,
        r#type: String,
        msg_increase: i32,
        from_id: i32,
        from_public_key: String,
        from_name: String,
        from_avatar: String,
        from_unread_num: i32,
        from_msg_id: i32,
        from_msg: String,
        from_update: String,
        ext: String,
        status: String,
    ) -> Self {
        Self {
            created_at: crate::utils::time::now(),
            user_id,
            r#type,
            msg_increase,
            from_id,
            from_public_key,
            from_name,
            from_avatar,
            from_unread_num,
            from_msg_id,
            from_msg,
            from_update,
            ext,
            status,
            ..Default::default()
        }
    }
}

impl resource::GenResourceID for Chat {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
