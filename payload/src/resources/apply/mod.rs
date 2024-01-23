pub mod reply;

use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Apply {
    pub id: u32,
    pub r#type: u8,
    pub type_id: u8,
    pub user_id: u32,
    pub content: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
