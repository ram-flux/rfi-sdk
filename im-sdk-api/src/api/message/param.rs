#[derive(Debug, serde::Serialize)]
pub struct PushMsgRes {
    pub(crate) message_id: u32,
}

impl PushMsgRes {
    pub async fn push_msg(
        content: String,
        mode: u8,
        from_id: u32,
        user_id: u32,
        chat_id: u32,
        chat_type: u8,
        _endpoint: String,
    ) -> Result<Self, crate::Error> {
        let message_id = crate::handler::message::push_msg(
            content, mode, from_id, user_id, chat_id, chat_type, _endpoint,
        )
        .await?;
        Ok(Self { message_id })
    }
}
