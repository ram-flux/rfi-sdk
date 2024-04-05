//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) struct ReplyApplyReq {
    apply_reply: payload::resources::apply::reply::ApplyReply,
    apply_reply_id: u32,
}

impl ReplyApplyReq {
    pub(crate) fn new(
        apply_reply: payload::resources::apply::reply::ApplyReply,
        apply_reply_id: u32,
    ) -> Self {
        Self {
            apply_reply,
            apply_reply_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::reply_apply(self.apply_reply, self.apply_reply_id).await?;
        Ok(())
    }
}

pub(crate) struct DeleteApplyReplyReq {
    apply_id: u32,
}

impl DeleteApplyReplyReq {
    pub(crate) fn new(apply_id: u32) -> Self {
        Self { apply_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_apply_reply(self.apply_id).await?;
        Ok(())
    }
}
