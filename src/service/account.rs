#[cfg(not(feature = "mock"))]
use resource::Action as _;

#[cfg(not(feature = "mock"))]
pub(crate) async fn new_account(account: payload::resources::account::Account, account_id: u32) {
    let id = payload::utils::gen_id();
    let account_action = resource::GeneralAction::Upsert {
        id: Some(account_id),
        resource: account,
    };

    let account_resource = crate::resources::Resources::Account(resource::Command::new(
        id as i64,
        account_action,
        "UpsertAccount".to_string(),
    ));

    let pool = crate::db::USER_SQLITE_POOL.read().await;
    let pool = pool.get_pool().unwrap();
    let _ = account_resource.execute(pool.as_ref()).await;
}

#[cfg(not(feature = "mock"))]
pub(crate) async fn update_account(account: payload::resources::account::Account, account_id: u32) {
    let id = payload::utils::gen_id();

    let account_action = resource::GeneralAction::Update {
        id: account_id,
        resource: account,
    };

    let account_resource = crate::resources::Resources::Account(resource::Command::new(
        id as i64,
        account_action,
        "UpdateAccount".to_string(),
    ));

    let pool = crate::db::USER_SQLITE_POOL.read().await;
    let pool = pool.get_pool().unwrap();
    let _ = account_resource.execute(pool.as_ref()).await;
}
