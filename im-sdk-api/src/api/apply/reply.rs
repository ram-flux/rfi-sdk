/// 回复申请(done, untested)
pub async fn reply_apply(
    apply_id: u32,
    // user_id: u32,
    content: String,
    status: u8,
) -> crate::response::Response<u32> {
    #[cfg(feature = "mock")]
    return Ok(123).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::apply::reply::reply_apply(apply_id, content, status)
            .await
            .into()
    }
}

/// 删除回复申请(done, untested)
pub async fn del_apply_reply(apply_reply_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        crate::handler::apply::reply::del_apply_reply(apply_reply_id)
            .await
            .into()
    }
}
