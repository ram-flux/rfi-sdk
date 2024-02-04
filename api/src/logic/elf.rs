use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct ElfDetailRes {
    pub id: u32,
    pub r#type: u8,
    pub token: String,
    pub name: String,
    pub avatar: String,
    pub ext: String,
    pub status: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ElfDetailRes {
    pub(crate) async fn detail(id: u32) -> Result<ElfDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        ElfDetailRes::query_one(async move |user_pool, pub_pool| {
            let sql = "SELECT id, type, token, name, avatar, ext, status,
                    created_at, updated_at 
                FROM elf 
                WHERE id =$1;";
            sqlx::query_as::<sqlx::Sqlite, ElfDetailRes>(sql)
                .bind(id)
                .fetch_one(user_pool.as_ref())
                .await
        })
        .await
        .map_err(Into::into)
    }

    // pub(crate) async fn list(
    //     r#type: u8,
    //     type_id: u32,
    //     page_size: u16,
    //     offset: u16,
    // ) -> Result<Vec<ElfDetailRes>, crate::SystemError> {
    //     use crate::operator::sqlite::query::Query as _;
    //     ElfDetailRes::query_all(async move |user_pool, pub_pool| {
    //         let sql = "SELECT id, type, token, name, avatar, ext, status,
    //             created_at, updated_at 
    //         FROM elf
    //         WHERE type = $1 AND type_id = $2
    //         LIMIT $2 OFFSET $3;";

    //         sqlx::query_as::<sqlx::Sqlite, ElfDetailRes>(sql)
    //             .bind(r#type)
    //             .bind(type_id)
    //             .bind(page_size)
    //             .bind(offset)
    //             .fetch_all(user_pool.as_ref())
    //             .await
    //     })
    //     .await
    //     .map_err(Into::into)
    // }
}
