use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Nav {
    pub id: u32,
    pub r#type: String,
    pub type_id: u32,
    pub user_id: u32,
    pub sort: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
