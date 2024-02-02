pub(crate) async fn send_message(
    message: payload::resources::message::Message,
    message_id: u32,
    recv_list: Vec<u32>,
) -> Result<(), crate::SystemError> {
    crate::logic::message::send_message(&message, message_id, recv_list).await?;
    crate::logic::message::save_message(message, message_id as u32).await?;
    Ok(())
}
