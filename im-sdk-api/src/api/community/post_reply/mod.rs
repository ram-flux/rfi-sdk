pub mod param;

/// 回复帖子(tested)
pub async fn reply_post(
    community_id: u32,
    post_id: u32,
    user_id: u32,
    content: String,
    sort: i32,
) -> crate::response::Response<param::PostReplyRes> {
    param::PostReplyRes::reply_post(community_id, post_id, user_id, content, sort)
        .await
        .into()
}

/// 更新帖子回复(done, untested)
// pub async fn update_post_reply(
//     post_reply_id: u32,
//     community_id: u32,
//     user_id: u32,
//     post_id: u32,
//     content: String,
//     sort: i32,
// ) -> crate::response::Response<()> {
//     #[cfg(feature = "mock")]
//     return Ok(()).into();
//     #[cfg(not(feature = "mock"))]
//     {
//         let post_reply = payload::resources::community::post_reply::PostReply::new(
//             community_id,
//             user_id,
//             post_id,
//             content,
//             sort,
//         );
//         crate::service::community::post_reply::UpdatePostReplyReq::new(post_reply, post_reply_id)
//             .exec()
//             .await?;
//         Ok(())
//     }
// }

/// 编辑帖子回复(tested)
pub async fn edit_post_reply(
    post_reply_id: u32,
    user_id: u32,
    content: String,
    _sort: i32,
) -> crate::response::Response<()> {
    crate::handler::community::post_reply::edit_post_reply(post_reply_id, user_id, content, _sort)
        .await
        .into()
}

/// 删除帖子回复(tested)
pub async fn del_post_reply(post_reply_id: u32) -> crate::response::Response<()> {
    crate::handler::community::post_reply::del_post_reply(post_reply_id)
        .await
        .into()
}

/// 回复帖子列表(tested)
pub async fn post_reply_list(
    post_id: u32,
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::community::post_reply::PostReplyDetailRes>> {
    crate::handler::community::post_reply::post_reply_list(post_id, page_size, offset)
        .await
        .into()
}

/// 帖子回复详情(tested)
pub async fn post_reply_detail(
    post_reply_id: u32,
) -> crate::response::Response<crate::logic::community::post_reply::PostReplyDetailRes> {
    crate::handler::community::post_reply::post_reply_detail(post_reply_id)
        .await
        .into()
}
