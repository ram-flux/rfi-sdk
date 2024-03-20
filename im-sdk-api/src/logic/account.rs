#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct AccountDetailRes {
    // pub id: u32,
    pub user_id: u32,
    pub public_key: String,
    pub account: String,
    pub salt: String,
    pub gender: u8,
    pub name: String,
    pub avatar: String,
    pub bio: String,
    pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    pub updated_at: Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
}

impl AccountDetailRes {
    pub(crate) async fn detail(user_id: u32) -> Result<AccountDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        AccountDetailRes::query_one(
            async move |user_pool: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>, _pub_pool| {
                let sql = "SELECT user_id, public_key, account, salt, gender, name, avatar,
                bio, created_at, updated_at 
                FROM account 
                WHERE user_id =$1;";
                sqlx::query_as::<sqlx::Sqlite, AccountDetailRes>(sql)
                    .bind(user_id)
                    .fetch_one(user_pool.as_ref())
                    .await
            },
        )
        .await
        .map_err(Into::into)
    }

    pub(crate) async fn list(
        page_size: u16,
        offset: u16,
    ) -> Result<Vec<AccountDetailRes>, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        AccountDetailRes::query_all(
            async move |user_pool: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>, _pub_pool| {
                let sql = "SELECT user_id, public_key, account, salt, gender, name, avatar,
                bio, created_at, updated_at 
            FROM account
            LIMIT $1 OFFSET $2;";

                sqlx::query_as::<sqlx::Sqlite, AccountDetailRes>(sql)
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
