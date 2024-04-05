//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod param;

/// 创建帖子(tested)
pub async fn create_post(
    community_id: u32,
    user_id: u32,
    name: String,
    content: String,
    sort_count: i32,
) -> crate::response::Response<param::CreatePostRes> {
    param::CreatePostRes::create_post(community_id, user_id, name, content, sort_count)
        .await
        .into()
}

// pub async fn update_post(
//     post_id: u32,
//     community_id: u32,
//     user_id: u32,
//     name: String,
//     content: String,
//     sort_count: i32,
// ) -> crate::response::Response<()> {
//     #[cfg(feature = "mock")]
//     return Ok(()).into();
//     // #[cfg(not(feature = "mock"))]
//     {
//         let post = payload::resources::community::post::Post::new(
//             community_id,
//             user_id,
//             name,
//             content,
//             sort_count,
//         );
//         let mut worker = crate::operator::WrapWorker::worker()?;
//         crate::service::community::post::UpdatePostReq::new(post, post_id)
//             .exec()
//             .await?;
//         Ok(())
//     }
// }

/// 编辑帖子(tested)
pub async fn edit_post(
    post_id: u32,
    name: String,
    content: String,
    _sort_count: i32,
) -> crate::response::Response<()> {
    crate::handler::community::post::edit_post(post_id, name, content, _sort_count)
        .await
        .into()
}

/// 删除帖子(tested)
pub async fn del_post(post_id: u32) -> crate::response::Response<()> {
    crate::handler::community::post::del_post(post_id)
        .await
        .into()
}

/// 帖子列表(done, untested)
pub async fn post_list(
    community_id: u32,
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::community::post::PostDetailRes>> {
    crate::handler::community::post::post_list(community_id, page_size, offset)
        .await
        .into()
}

/// 帖子详情(tested)
pub async fn post_detail(
    post_id: u32,
) -> crate::response::Response<crate::logic::community::post::PostDetailRes> {
    crate::handler::community::post::post_detail(post_id)
        .await
        .into()
}
