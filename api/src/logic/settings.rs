use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Default, serde::Serialize, sqlx::FromRow)]
pub struct SettingsDetailRes {
    pub id: u32,
    pub user_id: u32,
    pub language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl SettingsDetailRes {
    pub(crate) async fn detail(id: u32) -> Result<SettingsDetailRes, crate::SystemError> {
        use crate::operator::sqlite::query::Query as _;
        SettingsDetailRes::query_one(async move |user_pool, pub_pool| {
            let sql = "SELECT id, user_id, language,
                    created_at, updated_at 
                FROM settings 
                WHERE user_id =$1;";
            sqlx::query_as::<sqlx::Sqlite, SettingsDetailRes>(sql)
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
