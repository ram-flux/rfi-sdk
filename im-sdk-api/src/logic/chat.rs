use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct ChatDetailRes {
    pub id: u32,
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

impl ChatDetailRes {
    pub(crate) async fn detail(chat_id: u32) -> Result<ChatDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        ChatDetailRes::query_one(async move |user_pool, pub_pool| {
            let sql = "SELECT id, type, user_id, msg_increase, from_id,
                    from_public_key, from_name, from_avatar, from_unread_num, 
                    from_msg_id, from_msg, from_update, ext, status,
                    created_at, updated_at
                FROM chat
                WHERE id =$1;";
            sqlx::query_as::<sqlx::Sqlite, ChatDetailRes>(sql)
                .bind(chat_id)
                .fetch_one(user_pool.as_ref())
                .await
        })
        .await
        .map_err(Into::into)
    }

    pub(crate) async fn list(
        user_id: u32,
        page_size: u16,
        offset: u16,
    ) -> Result<Vec<ChatDetailRes>, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        ChatDetailRes::query_all(async move |user_pool, pub_pool| {
            let sql = "SELECT id, type, user_id, msg_increase, from_id,
                from_public_key, from_name, from_avatar, from_unread_num, 
                from_msg_id, from_msg, from_update, ext, status,
                created_at, updated_at
            FROM chat
            WHERE user_id = $1
            LIMIT $2 OFFSET $3;";

            sqlx::query_as::<sqlx::Sqlite, ChatDetailRes>(sql)
                .bind(user_id)
                .bind(page_size)
                .bind(offset)
                .fetch_all(user_pool.as_ref())
                .await
        })
        .await
        .map_err(Into::into)
    }
}
