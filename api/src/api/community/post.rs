/// 帖子列表(done, untested)
pub async fn post_list(
    community_id: u32,
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::community::post::PostDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::community::post::PostDetailRes {
                user_id: 6546,
                name: "test".to_string(),
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            crate::logic::community::post::PostDetailRes {
                user_id: 5435,
                name: "test2".to_string(),
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
            crate::service::community::post::PostListReq::new(community_id, page_size, offset)
                .exec()
                .await?,
        )
    }
}

/// 创建帖子(done, untested)
pub async fn create_post(
    community_id: u32,
    user_id: u32,
    name: String,
    content: String,
    sort_count: i32,
) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(3234).into();
    // #[cfg(not(feature = "mock"))]
    {
        let post = payload::resources::community::post::Post::new(
            community_id,
            user_id,
            name,
            content,
            sort_count,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        let post_id = worker.gen_id()?;
        crate::service::community::post::CreatePostReq::new(post, post_id)
            .exec()
            .await?;
        Ok(post_id)
    }
}

pub async fn update_post(
    post_id: u32,
    community_id: u32,
    user_id: u32,
    name: String,
    content: String,
    sort_count: i32,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        let post = payload::resources::community::post::Post::new(
            community_id,
            user_id,
            name,
            content,
            sort_count,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        crate::service::community::post::UpdatePostReq::new(post, post_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 删除帖子(done, untested)
pub async fn del_post(post_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::community::post::DeletePostReq::new(post_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 帖子详情(done, untested)
pub async fn post_detail(
    post_id: u32,
) -> Result<crate::logic::community::post::PostDetailRes, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let post = crate::logic::community::post::PostDetailRes {
            user_id: 6565656,
            ..Default::default()
        };
        return Ok(post);
    }
    // #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(crate::service::community::post::PostDetailReq::new(post_id)
            .exec()
            .await?)
    }
}
