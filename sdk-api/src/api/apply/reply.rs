//
//  Copyright 2024 Ram Flux, LLC.
//

/// 回复申请(done, untested)
pub async fn reply_apply(
    apply_id: u32,
    // user_id: u32,
    content: String,
    status: u8,
) -> crate::response::Response<super::param::ReplyApplyRes> {
    super::param::ReplyApplyRes::reply_apply(apply_id, content, status)
        .await
        .into()
}

/// 删除回复申请(done, untested)
pub async fn del_apply_reply(apply_reply_id: u32) -> crate::response::Response<()> {
    crate::handler::apply::reply::del_apply_reply(apply_reply_id)
        .await
        .into()
}
