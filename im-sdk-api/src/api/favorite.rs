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
        use crate::operator::sqlite::query::Query;
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        Ok(
            crate::service::favorite::FavoriteListReq::new(user.user_id, page_size, offset)
                .exec()
                .await?,
        )
    }
}

/// 添加收藏(done, untested)
pub async fn add_favorite(content: String) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(123);
    #[cfg(not(feature = "mock"))]
    {
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        let favorite = payload::resources::favorite::Favorite::new(user.user_id, content);
        let mut worker = crate::operator::WrapWorker::worker()?;
        let favorite_id = worker.gen_id()?;
        crate::service::favorite::AddFavorite::new(favorite, favorite_id)
            .exec()
            .await?;

        Ok(favorite_id)
    }
}

/// 删除收藏(done, untested)
pub async fn del_favorite(favorite_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(());
    #[cfg(not(feature = "mock"))]
    {
        crate::service::favorite::DeleteFavoriteReq::new(favorite_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 收藏详情(done, untested)
pub async fn favorite_detail(
    favorite_id: u32,
) -> Result<crate::logic::favorite::FavoriteDetailRes, crate::Error> {
    let favorite = crate::logic::favorite::FavoriteDetailRes {
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(favorite);
    #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::favorite::FavoriteDetailReq::new(favorite_id)
                .exec()
                .await?,
        )
    }
}
