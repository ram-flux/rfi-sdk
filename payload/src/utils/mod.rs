use self::snowflake::SnowflakeIdWorkerInner;

pub mod snowflake;
pub mod time;

pub fn gen_id() -> u32 {
    let mut id_worker = worker();
    id_worker.next_id().unwrap() as u32
}

pub fn worker() -> SnowflakeIdWorkerInner {
    crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap()
}
