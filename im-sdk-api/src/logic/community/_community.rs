use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct CommunityDetailRes {
    pub id: u32,
    pub father_id: Option<u32>,
    pub user_id: u32,
    pub name: String,
    pub bio: String,
    pub passwd: Option<String>,
    pub announcement: Option<String>,
    pub avatar: String,
    pub pinned: bool,
    pub status: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl CommunityDetailRes {
    pub(crate) async fn detail(
        community_id: u32,
    ) -> Result<CommunityDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        CommunityDetailRes::query_one(async move |user_pool, pub_pool| {
            let sql = "SELECT id, father_id, user_id, name, bio, passwd, announcement, avatar,
                    pinned, status, created_at, updated_at
                FROM community
                WHERE id =$1;";
            sqlx::query_as::<sqlx::Sqlite, CommunityDetailRes>(sql)
                .bind(community_id)
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
    ) -> Result<Vec<CommunityDetailRes>, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        CommunityDetailRes::query_all(async move |user_pool, pub_pool| {
            let sql = "SELECT id, father_id, user_id, name, bio, passwd, announcement, avatar,
                 pinned, status, created_at, updated_at
            FROM community
            WHERE user_id = $1
            LIMIT $2 OFFSET $3;";

            sqlx::query_as::<sqlx::Sqlite, CommunityDetailRes>(sql)
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
