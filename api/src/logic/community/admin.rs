use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct CommunityAdminDetailRes {
    pub id: u32,
    pub r#type: u8,
    pub community_id: u32,
    pub user_id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl CommunityAdminDetailRes {
    pub(crate) async fn detail(
        admin_id: u32,
    ) -> Result<CommunityAdminDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        CommunityAdminDetailRes::query_one(async move |user_pool, pub_pool| {
            let sql = "SELECT id, type, user_id, community_id,
                    created_at, updated_at
                FROM community_admin
                WHERE id =$1;";
            sqlx::query_as::<sqlx::Sqlite, CommunityAdminDetailRes>(sql)
                .bind(admin_id)
                .fetch_one(user_pool.as_ref())
                .await
        })
        .await
        .map_err(Into::into)
    }

    pub(crate) async fn list(
        community_id: u32,
        page_size: u16,
        offset: u16,
    ) -> Result<Vec<CommunityAdminDetailRes>, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        CommunityAdminDetailRes::query_all(async move |user_pool, pub_pool| {
            let sql = "SELECT id, type, user_id, community_id,
                created_at, updated_at
            FROM community
            WHERE user_id = $1
            LIMIT $2 OFFSET $3;";

            sqlx::query_as::<sqlx::Sqlite, CommunityAdminDetailRes>(sql)
                .bind(community_id)
                .bind(page_size)
                .bind(offset)
                .fetch_all(user_pool.as_ref())
                .await
        })
        .await
        .map_err(Into::into)
    }
}
