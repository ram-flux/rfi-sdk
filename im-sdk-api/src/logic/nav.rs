use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct NavDetailRes {
    pub id: u32,
    pub r#type: String,
    pub type_id: u32,
    pub user_id: u32,
    pub sort: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl NavDetailRes {
    pub(crate) async fn detail(id: u32) -> Result<NavDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        NavDetailRes::query_one(async move |user_pool, pub_pool| {
            let sql = "SELECT id, type, type_id, user_id, sort,
                    created_at, updated_at 
                FROM nav 
                WHERE id =$1;";
            sqlx::query_as::<sqlx::Sqlite, NavDetailRes>(sql)
                .bind(id)
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
    ) -> Result<Vec<NavDetailRes>, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        NavDetailRes::query_all(async move |user_pool, pub_pool| {
            let sql = "SELECT id, type, type_id, user_id, sort,
                created_at, updated_at 
            FROM nav
            WHERE user_id = $1 
            LIMIT $2 OFFSET $3;";

            sqlx::query_as::<sqlx::Sqlite, NavDetailRes>(sql)
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
