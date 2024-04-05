//
//  Copyright 2024 Ram Flux, LLC.
//

use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct PostReplyDetailRes {
    pub id: u32,
    pub community_id: u32,
    pub user_id: u32,
    pub post_id: u32,
    pub content: String,
    pub sort: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PostReplyDetailRes {
    pub(crate) async fn detail(post_id: u32) -> Result<PostReplyDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        PostReplyDetailRes::query_one(
            async move |user_pool: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>, _pub_pool| {
                let sql = "SELECT id, community_id, user_id, post_id, content, sort,
                    created_at, updated_at
                FROM community_post_reply
                WHERE id =$1;";
                sqlx::query_as::<sqlx::Sqlite, PostReplyDetailRes>(sql)
                    .bind(post_id)
                    .fetch_one(user_pool.as_ref())
                    .await
            },
        )
        .await
        .map_err(Into::into)
    }

    pub(crate) async fn list(
        post_id: u32,
        page_size: u16,
        offset: u16,
    ) -> Result<Vec<PostReplyDetailRes>, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        PostReplyDetailRes::query_all(
            async move |user_pool: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>, _pub_pool| {
                let sql = "SELECT id, community_id, user_id, post_id, content, sort,
                created_at, updated_at
            FROM community_post_reply
            WHERE post_id = $1
            LIMIT $2 OFFSET $3;";

                sqlx::query_as::<sqlx::Sqlite, PostReplyDetailRes>(sql)
                    .bind(post_id)
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
