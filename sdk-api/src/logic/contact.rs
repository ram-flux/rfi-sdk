//
//  Copyright 2024 Ram Flux, LLC.
//

use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct ContactDetailRes {
    pub id: u32,
    pub user_id: u32,
    pub friend_id: u32,
    pub friend_account: String,
    pub gender: u8,
    pub name: String,
    pub avatar: String,
    pub bio: String,
    pub remark: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ContactDetailRes {
    pub(crate) async fn detail(contact_id: u32) -> Result<ContactDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        ContactDetailRes::query_one(
            async move |user_pool: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>, _pub_pool| {
                let sql = "SELECT id, user_id, friend_id, friend_account,
                    gender, name, avatar, bio, 
                    remark, created_at, updated_at
                FROM contact
                WHERE id =$1;";
                sqlx::query_as::<sqlx::Sqlite, ContactDetailRes>(sql)
                    .bind(contact_id)
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
    ) -> Result<Vec<ContactDetailRes>, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        ContactDetailRes::query_all(
            async move |user_pool: std::sync::Arc<sqlx::Pool<sqlx::Sqlite>>, _pub_pool| {
                let sql = "SELECT id, user_id, friend_id, friend_account,
                gender, name, avatar, bio, 
                remark, created_at, updated_at
            FROM contact
            WHERE user_id = $1
            LIMIT $2 OFFSET $3;";

                sqlx::query_as::<sqlx::Sqlite, ContactDetailRes>(sql)
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
