use resource::Action as _;

pub(crate) async fn save_message(account: payload::resources::message::Message, account_id: u32) {
    let id = payload::utils::gen_id();
    let message_action = resource::GeneralAction::Upsert {
        id: Some(account_id),
        resource: account,
    };

    let account_resource = crate::resources::Resources::Message(resource::Command::new(
        id as i64,
        message_action,
        "UpsertAccount".to_string(),
    ));

    let pool = crate::operator::sqlite::init::USER_SQLITE_POOL.read().await;
    let pool = pool.get_pool().unwrap();
    let res = account_resource.execute(pool.as_ref()).await;
    println!("[new_message] res: {res:?}");
}
