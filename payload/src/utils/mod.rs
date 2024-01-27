pub mod snowflake;
pub mod time;

pub fn gen_id() -> u32 {
    let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
    id_worker.next_id().unwrap() as u32
}
