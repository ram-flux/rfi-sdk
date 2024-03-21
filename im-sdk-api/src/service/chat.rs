pub(crate) struct UpdateChatStatusReq {
    chat_status: payload::resources::chat::status::ChatStatus,
    chat_id: u32,
}

impl UpdateChatStatusReq {
    pub(crate) fn new(
        chat_status: payload::resources::chat::status::ChatStatus,
        chat_id: u32,
    ) -> Self {
        Self {
            chat_status,
            chat_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_chat_status(self.chat_status, self.chat_id).await?;
        Ok(())
    }
}

pub(crate) struct ChatListReq {
    user_id: u32,
    page_size: u16,
    offset: u16,
}

impl ChatListReq {
    pub(crate) fn new(user_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            user_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<Vec<crate::logic::chat::ChatDetailRes>, crate::SystemError> {
        crate::logic::chat::ChatDetailRes::list(self.user_id, self.page_size, self.offset).await
    }
}
