#[cfg(not(feature = "mock"))]
use payload::utils::worker;
#[cfg(not(feature = "mock"))]
use resource::Action as _;

#[cfg(not(feature = "mock"))]
pub(crate) async fn new_device(device: payload::resources::device::Device, device_id: u32) {
    let mut worker = worker();
    let trace_id = worker.next_id().unwrap() as u32;
    let device_action = resource::GeneralAction::Upsert {
        id: Some(device_id),
        resource: device,
    };

    let device_resource = crate::resources::Resources::Device(resource::Command::new(
        trace_id as i64,
        device_action,
        "UpsertDevice".to_string(),
    ));

    let pool = crate::db::USER_SQLITE_POOL.read().await;
    let pool = pool.get_pool().unwrap();
    let res = device_resource.execute(pool.as_ref()).await;
    println!("[new_device] res: {res:?}");
}

#[cfg(not(feature = "mock"))]
pub(crate) async fn del_device(device_id: u32) {
    let mut worker = worker();
    let trace_id = worker.next_id().unwrap() as u32;
    let device_action = resource::GeneralAction::Drop(device_id);

    let device_resource = crate::resources::Resources::Device(resource::Command::new(
        trace_id as i64,
        device_action,
        "DropDevice".to_string(),
    ));

    let pool = crate::db::USER_SQLITE_POOL.read().await;
    let pool = pool.get_pool().unwrap();
    let res = device_resource.execute(pool.as_ref()).await;
    println!("[del_device] res: {res:?}");
}

#[cfg(not(feature = "mock"))]
pub(crate) async fn update_token(token: payload::resources::device::token::Token, device_id: u32) {
    let mut worker = worker();
    let trace_id = worker.next_id().unwrap() as u32;
    let token_action = resource::GeneralAction::Update {
        id: device_id,
        resource: token,
    };

    let token_resource = crate::resources::Resources::Token(resource::Command::new(
        trace_id as i64,
        token_action,
        "UpdateToken".to_string(),
    ));

    let pool = crate::db::USER_SQLITE_POOL.read().await;
    let pool = pool.get_pool().unwrap();
    let res = token_resource.execute(pool.as_ref()).await;
    println!("[update_token] res: {res:?}");
}
