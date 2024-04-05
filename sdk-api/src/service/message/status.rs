//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) struct UpdateMessageStatusReq {
    message_status: payload::resources::message::status::MessageStatus,
    message_id: u32,
}

impl UpdateMessageStatusReq {
    pub(crate) fn new(
        message_status: payload::resources::message::status::MessageStatus,
        message_id: u32,
    ) -> Self {
        Self {
            message_status,
            message_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_message_status(self.message_status, self.message_id).await?;
        Ok(())
    }
}
