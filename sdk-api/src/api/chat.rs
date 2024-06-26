//
//  Copyright 2024 Ram Flux, LLC.
//

/// 会话列表(done, untested)
pub async fn chat_list(
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::chat::ChatDetailRes>> {
    crate::handler::chat::chat_list(page_size, offset)
        .await
        .into()
}

pub async fn search_chat(
    _chat_id: u32,
    _keyword: String,
) -> crate::response::Response<Vec<crate::logic::community::admin::CommunityAdminDetailRes>> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::community::admin::CommunityAdminDetailRes {
                user_id: 6546,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            crate::logic::community::admin::CommunityAdminDetailRes {
                user_id: 5435,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
        ];
        return Ok(list).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}

/// 置顶会话(done, untested)
pub async fn pin_chat(chat_id: u32) -> crate::response::Response<()> {
    crate::handler::chat::pin_chat(chat_id).await.into()
}

/// 显示会话(done, untested)
pub async fn show_chat(chat_id: u32) -> crate::response::Response<()> {
    crate::handler::chat::show_chat(chat_id).await.into()
}

/// 隐藏会话(done, untested)
pub async fn hide_chat(chat_id: u32) -> crate::response::Response<()> {
    crate::handler::chat::hide_chat(chat_id).await.into()
}
