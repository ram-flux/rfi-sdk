//
//  Copyright 2024 Ram Flux, LLC.
//

#[derive(Debug, thiserror::Error)]
pub enum CommunityError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::error::common::database::DatabaseError),
}

impl CommunityError {
    // pub(crate) fn get_status_code(&self) -> u32 {
    //     match self {
    //         InitDatabaseError::DatabaseError(msg) => msg.get_status_code(),
    //     }
    // }
}
