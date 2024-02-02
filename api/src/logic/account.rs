#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct AccountDetailRes {
    pub id: u32,
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
    pub(crate) async fn exec(
        user_id: u32,
    ) -> Result<crate::operator::sqlite::query::QueryResult<AccountDetailRes>, crate::SystemError>
    {
        use crate::operator::sqlite::query::Query as _;
        AccountDetailRes::query(async move |user_pool, pub_pool| {
            let sql = "SELECT id, user_id, public_key, account, salt, gender, name, avatar,
                bio, created_at, updated_at 
                FROM account 
                WHERE user_id =$1;";
            sqlx::query_as::<sqlx::Sqlite, AccountDetailRes>(sql)
                .bind(user_id)
                .fetch_one(user_pool.as_ref())
                .await
                .map(Into::into)
        })
        .await
        .map_err(Into::into)
    }
}
