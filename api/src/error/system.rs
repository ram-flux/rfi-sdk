#[derive(Debug, thiserror::Error)]
pub enum SystemError {
    #[error("Database error: {0}")]
    Database(#[from] super::common::database::DatabaseError),
    #[error("parse failed: {0:?}")]
    Parse(String),
}

impl SystemError {
    pub fn get_status_code(&self) -> u32 {
        match self {
            SystemError::Database(_) => 6300,
            SystemError::Parse(_) => 6300,
        }
    }
}
