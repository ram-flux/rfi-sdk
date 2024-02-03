use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct PostDetailRes {
    pub id: u32,
    pub community_id: u32,
    pub user_id: u32,
    pub name: String,
    pub content: String,
    pub sort_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PostDetailRes {
    pub(crate) async fn detail(
        post_id: u32,
    ) -> Result<crate::operator::sqlite::query::QueryResult<PostDetailRes>, crate::SystemError>
    {
        use crate::operator::sqlite::query::Query as _;
        PostDetailRes::query(async move |user_pool, pub_pool| {
            let sql = "SELECT id, community_id, user_id, name, content, sort_count
                    created_at, updated_at
                FROM community_post
                WHERE id =$1;";
            sqlx::query_as::<sqlx::Sqlite, PostDetailRes>(sql)
                .bind(post_id)
                .fetch_one(user_pool.as_ref())
                .await
                .map(Into::into)
        })
        .await
        .map_err(Into::into)
    }

    pub(crate) async fn list(
        community_id: u32,
        page_size: u16,
        offset: u16,
    ) -> Result<crate::operator::sqlite::query::QueryResult<PostDetailRes>, crate::SystemError>
    {
        use crate::operator::sqlite::query::Query as _;
        PostDetailRes::query(async move |user_pool, pub_pool| {
            let sql = "SELECT id, community_id, user_id, name, content, sort_count
                created_at, updated_at
            FROM community_post
            WHERE community_id = $1
            LIMIT $2 OFFSET $3;";

            sqlx::query_as::<sqlx::Sqlite, PostDetailRes>(sql)
                .bind(community_id)
                .bind(page_size)
                .bind(offset)
                .fetch_all(user_pool.as_ref())
                .await
                .map(Into::into)
        })
        .await
        .map_err(Into::into)
    }
}
