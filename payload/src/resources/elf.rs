use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Elf {
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
