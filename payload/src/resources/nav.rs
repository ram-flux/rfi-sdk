use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Nav {
    pub id: i32,
    pub r#type: String,
    pub type_id: i32,
    pub user_id: i32,
    pub sort: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
