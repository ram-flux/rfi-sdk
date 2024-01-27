#[derive(Debug, thiserror::Error)]
pub enum BadRequest {
    #[error("Init database error: {0}")]
    InitDatabase(#[from] super::api::init_database::InitDatabaseError),
    #[error("Community error: {0}")]
    Community(#[from] super::api::community::CommunityError),
}

impl BadRequest {
    // pub(crate) fn get_status_code(&self) -> u32 {
    //     match self {
    //         BadRequest::InitDatabase(msg) => msg.get_status_code(),
    //     }
    // }
}
