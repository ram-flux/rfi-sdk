use std::ops::{Deref, DerefMut};

pub(crate) mod net;
pub(crate) mod sqlite;

pub(crate) struct WrapWorker(payload::utils::snowflake::SnowflakeIdWorkerInner);

impl Deref for WrapWorker {
    fn deref(&self) -> &Self::Target {
        &self.0
    }

    type Target = payload::utils::snowflake::SnowflakeIdWorkerInner;
}

impl DerefMut for WrapWorker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WrapWorker {
    pub(crate) fn worker() -> Result<Self, crate::SystemError> {
        payload::utils::worker().map(Self).map_err(Into::into)
    }

    pub fn gen_id(&mut self) -> Result<u32, crate::SystemError> {
        self.next_id().map_err(Into::into).map(|id| id as u32)
    }

    pub fn gen_trace_id(&mut self) -> Result<i64, crate::SystemError> {
        self.next_id().map_err(Into::into).map(|id| id as i64)
    }
}
