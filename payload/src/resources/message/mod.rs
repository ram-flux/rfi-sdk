pub mod block;

use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Message {
    pub id: u32,
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
