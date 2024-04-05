//
//  Copyright 2024 Ram Flux, LLC.
//

// net send
pub(crate) async fn send_message(
    message: &payload::resources::message::Message,
    _message_id: u32,
    recv_list: Vec<u32>,
) -> Result<(), crate::SystemError> {
    let mut worker = crate::operator::WrapWorker::worker()?;
    let trace_id = worker.gen_trace_id()?;

    let data = {
        let data = bson::to_vec(message).map_err(|e| crate::SystemError::Serde(e.into()))?;
        std::sync::Arc::new(data)
    };

    let list: Vec<[u8; 32]> = Vec::new();
    let msg = im_net::Packet::message(trace_id as u64, list, data);

    let tx = crate::operator::net::channel::net_channel_generator();
    tx.send(crate::operator::net::channel::Event::Send {
        from_id: message.from_id,
        recv_list,
        data: msg,
    })
    .map_err(|e| crate::SystemError::Net(e.into()))?;

    Ok(())
}
