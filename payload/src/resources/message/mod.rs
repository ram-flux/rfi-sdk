pub mod block;

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
    pg_table_name = "message",
    sqlite_table_name = "message",
    primary_key = "id:u32",
    constraint = "im_message_id_idx"
)]
pub struct Message {
    // text/文本/1 emoti/表情/2 file/文件/3 image/图片/5 video/视频/6 audio/音频/7 帖子/post/8 精灵/elf/9
    pub mode: u8,
    pub from_id: u32,
    pub user_id: u32,
    pub chat_id: u32,
    pub chat_type: u8,
    pub datas: String,
    pub has_read: u8,
    pub msg_error: u8,
    pub status: u8,
    pub send_time: DateTime<Utc>,
    pub accept_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Message {
    pub fn new(
        mode: u8,
        from_id: u32,
        user_id: u32,
        chat_id: u32,
        chat_type: u8,
        datas: String,
        has_read: u8,
        msg_error: u8,
        status: u8,
        send_time: DateTime<Utc>,
        accept_time: DateTime<Utc>,
    ) -> Self {
        Self {
            mode,
            from_id,
            user_id,
            chat_id,
            chat_type,
            datas,
            has_read,
            msg_error,
            send_time,
            accept_time,
            status,
            created_at: crate::utils::time::now(),
            updated_at: Some(crate::utils::time::now()),
        }
    }
}

impl resource::GenResourceID for Message {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}
