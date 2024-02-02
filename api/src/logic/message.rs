use resource::Action as _;

// net send
pub(crate) async fn send_message(
    message: &payload::resources::message::Message,
    message_id: u32,
    recv_list: Vec<u32>,
) -> Result<(), crate::SystemError> {
    // #[cfg(feature = "mock")]
    let mut worker = crate::operator::WrapWorker::worker()?;
    let trace_id = worker.gen_trace_id()?;

    let data = {
        let data = bson::to_vec(message).map_err(|e| crate::SystemError::Serde(e.into()))?;
        std::sync::Arc::new(data)
    };

    let list: Vec<[u8; 32]> = Vec::new();
    let msg = im_codec::Protocol::message(trace_id as u64, list, data);

    let tx = crate::operator::net::channel::net_channel_generator();
    tx.send(crate::operator::net::channel::Event::Send {
        from_id: message.from_id,
        recv_list,
        data: msg,
    })
    .map_err(|e| crate::SystemError::Net(e.into()))?;

    Ok(())
}

// db save
pub(crate) async fn save_message(
    message: payload::resources::message::Message,
    message_id: u32,
) -> Result<(), crate::SystemError> {
    let id = crate::operator::WrapWorker::worker()?.gen_trace_id()?;
    let message_action = resource::GeneralAction::Upsert {
        id: Some(message_id),
        resource: message,
    };

    let account_resource = crate::resources::Resources::Message(resource::Command::new(
        id,
        message_action,
        "UpsertAccount".to_string(),
    ));

    let pool = crate::operator::sqlite::init::USER_SQLITE_POOL.read().await;
    let pool = pool.get_pool()?;
    let res = account_resource.execute(pool.as_ref()).await;
    println!("[new_message] res: {res:?}");
    Ok(())
}

// let recv_list: Vec<[u8; 32]> = recv_list
//     .iter()
//     .map(|&x| {
//         let bytes: [u8; 32] = x.to_le_bytes();
//         bytes
//     })
//     .collect();
