use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct FavoriteDetailRes {
    pub id: u32,
    pub user_id: u32,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl FavoriteDetailRes {
    pub(crate) async fn detail(id: u32) -> Result<FavoriteDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        FavoriteDetailRes::query_one(
            async move |user_pool: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>, _pub_pool| {
                let sql = "SELECT id, user_id, content,
                    created_at, updated_at 
                FROM favorite 
                WHERE id =$1;";
                sqlx::query_as::<sqlx::Sqlite, FavoriteDetailRes>(sql)
                    .bind(id)
                    .fetch_one(user_pool.as_ref())
                    .await
            },
        )
        .await
        .map_err(Into::into)
    }

    pub(crate) async fn list(
        user_id: u32,
        page_size: u16,
        offset: u16,
    ) -> Result<Vec<FavoriteDetailRes>, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        FavoriteDetailRes::query_all(
            async move |user_pool: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>, _pub_pool| {
                let sql = "SELECT id, user_id, content,
                created_at, updated_at 
            FROM favorite
            WHERE user_id = $1 
            LIMIT $2 OFFSET $3;";

                sqlx::query_as::<sqlx::Sqlite, FavoriteDetailRes>(sql)
                    .bind(user_id)
                    .bind(page_size)
                    .bind(offset)
                    .fetch_all(user_pool.as_ref())
                    .await
            },
        )
        .await
        .map_err(Into::into)
    }
}
