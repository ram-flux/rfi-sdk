#[derive(Debug, thiserror::Error)]
pub enum AccountError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::error::common::database::DatabaseError),
}

impl AccountError {
    // pub(crate) fn get_status_code(&self) -> u32 {
    //     match self {
    //         InitDatabaseError::DatabaseError(msg) => msg.get_status_code(),
    //     }
    // }
}
