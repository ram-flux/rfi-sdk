//
//  Copyright 2024 Ram Flux, LLC.
//

use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct CommunityMemberDetailRes {
    pub id: u32,
    pub r#type: u8,
    pub user_id: u32,
    pub community_id: u32,
    pub name: String,
    pub avatar: String,
    pub sort: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl CommunityMemberDetailRes {
    pub(crate) async fn detail(
        community_id: u32,
        member_id: u32,
    ) -> Result<CommunityMemberDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        CommunityMemberDetailRes::query_one(
            async move |user_pool: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>, _pub_pool| {
                let sql = "SELECT id, type, user_id, community_id, name, avatar, sort,
                    created_at, updated_at
                FROM community_member
                WHERE user_id =$1 AND community_id =$2;";
                sqlx::query_as::<sqlx::Sqlite, CommunityMemberDetailRes>(sql)
                    .bind(member_id)
                    .bind(community_id)
                    .fetch_one(user_pool.as_ref())
                    .await
            },
        )
        .await
        .map_err(Into::into)
    }

    pub(crate) async fn list(
        community_id: u32,
        page_size: u16,
        offset: u16,
    ) -> Result<Vec<CommunityMemberDetailRes>, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        CommunityMemberDetailRes::query_all(
            async move |user_pool: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>, _pub_pool| {
                let sql = "SELECT id, type, user_id, community_id, name, avatar, sort,
                created_at, updated_at
            FROM community_member
            WHERE community_id = $1
            LIMIT $2 OFFSET $3;";

                sqlx::query_as::<sqlx::Sqlite, CommunityMemberDetailRes>(sql)
                    .bind(community_id)
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
