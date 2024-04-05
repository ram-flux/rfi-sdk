//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) mod status;
pub(crate) struct SendMessageReq {
    message: payload::resources::message::Message,
    message_id: u32,
    recv_list: Vec<u32>,
}

impl SendMessageReq {
    pub(crate) fn new(
        message: payload::resources::message::Message,
        message_id: u32,
        recv_list: Vec<u32>,
    ) -> Self {
        Self {
            message,
            message_id,
            recv_list,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::message::send_message(&self.message, self.message_id, self.recv_list).await?;
        crate::logic::upsert::new_message(self.message, self.message_id).await?;
        Ok(())
    }
}

pub(crate) struct UpdateMessageReq {
    message: payload::resources::message::Message,
    message_id: u32,
}

impl UpdateMessageReq {
    pub(crate) fn new(message: payload::resources::message::Message, message_id: u32) -> Self {
        Self {
            message,
            message_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_message(self.message, self.message_id).await?;
        Ok(())
    }
}

pub(crate) struct DeleteMessageReq {
    message_id: u32,
}

impl DeleteMessageReq {
    pub(crate) fn new(message_id: u32) -> Self {
        Self { message_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_message(self.message_id).await?;
        Ok(())
    }
}
