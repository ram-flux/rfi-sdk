//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod snowflake;

#[derive(serde::Serialize, thiserror::Error, Debug, Clone)]
pub enum PayloadError {
    #[error("Snowflake error: {0}")]
    Snowflake(#[from] snowflake::SnowflakeError),
}
