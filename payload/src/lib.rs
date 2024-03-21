#![feature(async_closure)]
#![allow(refining_impl_trait)]
#![allow(clippy::too_many_arguments)]
pub mod error;
pub mod resources;
pub mod utils;

use error::{snowflake::SnowflakeError, PayloadError};
