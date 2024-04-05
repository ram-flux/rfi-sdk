//
//  Copyright 2024 Ram Flux, LLC.
//

/// 回复申请(done, untested)
pub async fn reply_apply(
    apply_id: u32,
    // user_id: u32,
    content: String,
    status: u8,
) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(123).into();
    #[cfg(not(feature = "mock"))]
    {
        let detail = crate::service::apply::ApplyDetailReq::new(apply_id)
            .exec()
            .await?;

        let apply_reply = payload::resources::apply::reply::ApplyReply::new(
            apply_id,
            detail.type_id,
            detail.user_id,
            content,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        let apply_reply_id = worker.gen_id()?;
        crate::service::apply::reply::ReplyApplyReq::new(apply_reply, apply_reply_id)
            .exec()
            .await?;

        Ok(apply_reply_id)
    }
}

/// 删除回复申请(done, untested)
pub async fn del_apply_reply(apply_reply_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        crate::service::apply::reply::DeleteApplyReplyReq::new(apply_reply_id)
            .exec()
            .await?;
        Ok(())
    }
}
