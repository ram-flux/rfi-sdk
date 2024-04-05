//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod api;
pub mod bad_request;
pub mod common;
pub mod system;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // 请求错误
    #[error("{0}")]
    BadRequest(#[from] bad_request::BadRequest),
    // 内部错误
    #[error("Server error: {0}")]
    System(#[from] system::SystemError),
    // 鉴权错误
    #[error("Unauthorized")]
    UnAuthorize,

    #[error("parse failed: {0:?}")]
    Parse(String),
    #[error("Database error: {0}")]
    Database(#[from] common::database::DatabaseError),
    #[error("Wallet error: {0}")]
    Wallet(#[from] wallet::Error),
}
