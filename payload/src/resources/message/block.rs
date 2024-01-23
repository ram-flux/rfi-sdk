use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Block {
    pub id: i32,
    pub chat_id: i32,
    pub user_id: i32,
    pub blocks: Vec<Block>, // Assuming Block is another struct or a specific type
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
