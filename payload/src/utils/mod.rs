//
//  Copyright 2024 Ram Flux, LLC.
//

use self::snowflake::SnowflakeIdWorkerInner;

pub mod snowflake;
pub mod time;

pub fn worker() -> Result<SnowflakeIdWorkerInner, crate::PayloadError> {
    crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1)
}
