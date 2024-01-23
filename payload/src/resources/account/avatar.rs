use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Default)]
pub struct Avatar {
    pub bio: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
