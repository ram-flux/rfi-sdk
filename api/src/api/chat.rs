/// 会话列表(done, untested)
pub async fn chat_list(
    user_id: u32,
    page_size: u16,
    offset: u16,
) -> Result<
    crate::operator::sqlite::query::QueryResult<crate::logic::chat::ChatDetailRes>,
    crate::Error,
> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::chat::ChatDetailRes {
                user_id: 6546,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            crate::logic::chat::ChatDetailRes {
                user_id: 5435,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
        ];
        return Ok(crate::operator::sqlite::query::QueryResult::Vec(list));
    }
    #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::chat::ChatListReq::new(user_id, page_size, offset)
                .exec()
                .await?,
        )
    }
}

pub async fn search_chat(
    chat_id: u32,
    keyword: String,
) -> Result<
    crate::operator::sqlite::query::QueryResult<
        crate::logic::community::admin::CommunityAdminDetailRes,
    >,
    crate::Error,
> {
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
        return Ok(crate::operator::sqlite::query::QueryResult::Vec(list));
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}

/// 置顶会话(done, untested)
pub async fn pin_chat(chat_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let chat_status = payload::resources::chat::status::ChatStatus::new(3);
        crate::service::chat::UpdateChatStatusReq::new(chat_status, chat_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 显示会话(done, untested)
pub async fn show_chat(chat_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let chat_status = payload::resources::chat::status::ChatStatus::new(2);
        crate::service::chat::UpdateChatStatusReq::new(chat_status, chat_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 隐藏会话(done, untested)
pub async fn hide_chat(chat_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let chat_status = payload::resources::chat::status::ChatStatus::new(1);
        crate::service::chat::UpdateChatStatusReq::new(chat_status, chat_id)
            .exec()
            .await?;
        Ok(())
    }
}
