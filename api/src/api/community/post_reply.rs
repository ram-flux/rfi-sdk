/// 回复帖子列表(done, untested)
pub async fn post_reply_list(
    post_id: u32,
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::community::post_reply::PostReplyDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::community::post_reply::PostReplyDetailRes {
                user_id: 6565656,
                post_id: 2343,
                content: "test".to_string(),
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            crate::logic::community::post_reply::PostReplyDetailRes {
                user_id: 6565656,
                post_id: 2343,
                content: "test".to_string(),
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
        ];
        return Ok(list);
    }
    #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::community::post_reply::PostReplyListReq::new(
                post_id, page_size, offset,
            )
            .exec()
            .await?,
        )
    }
}

/// 回复帖子(done, untested)
pub async fn reply_post(
    community_id: u32,
    post_id: u32,
    user_id: u32,
    content: String,
    sort: i32,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let post_reply = payload::resources::community::post_reply::PostReply::new(
            community_id,
            user_id,
            post_id,
            content,
            sort,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        let post_reply_id = worker.gen_id()?;
        crate::service::community::post_reply::ReplyPostReq::new(post_reply, post_reply_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 更新帖子回复(done, untested)
pub async fn update_post_reply(
    post_reply_id: u32,
    community_id: u32,
    user_id: u32,
    post_id: u32,
    content: String,
    sort: i32,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let post_reply = payload::resources::community::post_reply::PostReply::new(
            community_id,
            user_id,
            post_id,
            content,
            sort,
        );
        crate::service::community::post_reply::UpdatePostReplyReq::new(post_reply, post_reply_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 删除帖子回复(done, untested)
pub async fn del_post_reply(post_reply_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::community::post_reply::DeletePostReplyReq::new(post_reply_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 帖子回复详情(done, untested)
pub async fn post_reply_detail(
    post_reply_id: u32,
) -> Result<crate::logic::community::post_reply::PostReplyDetailRes, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let post_reply = crate::logic::community::post_reply::PostReplyDetailRes {
            user_id: 6565656,
            post_id: 2343,
            content: "test".to_string(),
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(post_reply);
    }
    // #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::community::post_reply::PostReplyDetailReq::new(post_reply_id)
                .exec()
                .await?,
        )
    }
}
