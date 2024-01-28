pub mod api;
pub mod bad_request;
pub mod common;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    BadRequest(#[from] bad_request::BadRequest),
    #[error("parse failed: {0:?}")]
    Parse(String),
    #[error("Database error: {0}")]
    Database(#[from] common::database::DatabaseError),
    #[error("Wallet error: {0}")]
    Wallet(#[from] wallet::Error),

    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}
