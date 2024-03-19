use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct ApplyDetailRes {
    pub id: u32,
    pub r#type: u8,
    pub type_id: u8,
    pub user_id: u32,
    pub content: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ApplyDetailRes {
    pub(crate) async fn detail(id: u32) -> Result<ApplyDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        ApplyDetailRes::query_one(async move |user_pool, pub_pool| {
            let sql = "SELECT id, type, type_id, user_id, content, status,
                    created_at, updated_at 
                FROM apply 
                WHERE id =$1;";
            sqlx::query_as::<sqlx::Sqlite, ApplyDetailRes>(sql)
                .bind(id)
                .fetch_one(user_pool.as_ref())
                .await
        })
        .await
        .map_err(Into::into)
    }

    pub(crate) async fn list(
        r#type: u8,
        type_id: u32,
        page_size: u16,
        offset: u16,
    ) -> Result<Vec<ApplyDetailRes>, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        ApplyDetailRes::query_all(async move |user_pool, pub_pool| {
            let sql = "SELECT id, type, type_id, user_id, content, status,
                created_at, updated_at 
            FROM apply
            WHERE type = $1 AND type_id = $2
            LIMIT $2 OFFSET $3;";

            sqlx::query_as::<sqlx::Sqlite, ApplyDetailRes>(sql)
                .bind(r#type)
                .bind(type_id)
                .bind(page_size)
                .bind(offset)
                .fetch_all(user_pool.as_ref())
                .await
        })
        .await
        .map_err(Into::into)
    }
}
