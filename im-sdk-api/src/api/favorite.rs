/// 收藏列表(done, untested)
pub async fn favorite_list(
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::favorite::FavoriteDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let msgs = vec![
            crate::logic::favorite::FavoriteDetailRes {
                user_id: 123123,
                ..Default::default()
            },
            crate::logic::favorite::FavoriteDetailRes {
                user_id: 123123,
                ..Default::default()
            },
        ];
        return Ok(msgs);
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::favorite::favorite_list(page_size, offset)
            .await
            .into()
    }
}

/// 添加收藏(done, untested)
pub async fn add_favorite(content: String) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(123);
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::favorite::add_favorite(content).await.into()
    }
}

/// 删除收藏(done, untested)
pub async fn del_favorite(favorite_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(());
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::favorite::del_favorite(favorite_id)
            .await
            .into()
    }
}

/// 收藏详情(done, untested)
pub async fn favorite_detail(
    favorite_id: u32,
) -> Result<crate::logic::favorite::FavoriteDetailRes, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let favorite = crate::logic::favorite::FavoriteDetailRes {
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(favorite);
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::favorite::favorite_detail(favorite_id)
            .await
            .into()
    }
}
