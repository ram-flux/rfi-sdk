pub(crate) struct ReplyPostReq {
    post_reply: payload::resources::community::post_reply::PostReply,
    post_reply_id: u32,
}

impl ReplyPostReq {
    pub(crate) fn new(
        post_reply: payload::resources::community::post_reply::PostReply,
        post_reply_id: u32,
    ) -> Self {
        Self {
            post_reply,
            post_reply_id,
        }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::reply_post(self.post_reply, self.post_reply_id).await?;
        Ok(())
    }
}

// pub(crate) struct UpdatePostReplyReq {
//     post_reply: payload::resources::community::post_reply::PostReply,
//     post_reply_id: u32,
// }

// impl UpdatePostReplyReq {
//     pub(crate) fn new(
//         post_reply: payload::resources::community::post_reply::PostReply,
//         post_reply_id: u32,
//     ) -> Self {
//         Self {
//             post_reply,
//             post_reply_id,
//         }
//     }

//     pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
//         crate::logic::update::update_post_reply(self.post_reply, self.post_reply_id).await?;
//         Ok(())
//     }
// }

pub(crate) struct EditPostReplyReq {
    post_reply_info: payload::resources::community::post_reply::info::PostReplyInfo,
    post_reply_id: u32,
}

impl EditPostReplyReq {
    pub(crate) fn new(
        post_reply_info: payload::resources::community::post_reply::info::PostReplyInfo,
        post_reply_id: u32,
    ) -> Self {
        Self {
            post_reply_info,
            post_reply_id,
        }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::edit_post_reply(self.post_reply_info, self.post_reply_id).await?;
        Ok(())
    }
}

pub(crate) struct PostReplyDetailReq {
    post_reply_id: u32,
}

impl PostReplyDetailReq {
    pub(crate) fn new(post_reply_id: u32) -> Self {
        Self { post_reply_id }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<crate::logic::community::post_reply::PostReplyDetailRes, crate::SystemError> {
        crate::logic::community::post_reply::PostReplyDetailRes::detail(self.post_reply_id).await
    }
}

pub(crate) struct PostReplyListReq {
    post_id: u32,
    page_size: u16,
    offset: u16,
}

impl PostReplyListReq {
    pub(crate) fn new(post_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            post_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<Vec<crate::logic::community::post_reply::PostReplyDetailRes>, crate::SystemError>
    {
        crate::logic::community::post_reply::PostReplyDetailRes::list(
            self.post_id,
            self.page_size,
            self.offset,
        )
        .await
    }
}

pub(crate) struct DeletePostReplyReq {
    post_reply_id: u32,
}

impl DeletePostReplyReq {
    pub(crate) fn new(post_reply_id: u32) -> Self {
        Self { post_reply_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_post_reply(self.post_reply_id).await?;
        Ok(())
    }
}
