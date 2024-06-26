//
//  Copyright 2024 Ram Flux, LLC.
//

/// 菜单列表(done, untested)
pub async fn nav_list(
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::nav::NavDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let msgs = vec![
            crate::logic::nav::NavDetailRes {
                user_id: 123123,
                ..Default::default()
            },
            crate::logic::nav::NavDetailRes {
                user_id: 123123,
                ..Default::default()
            },
        ];
        return Ok(msgs);
    }
    #[cfg(not(feature = "mock"))]
    {
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        Ok(
            crate::service::nav::NavListReq::new(user.user_id, page_size, offset)
                .exec()
                .await?,
        )
    }
}

/// 添加菜单(done, untested)
pub async fn add_nav(r#type: u8, type_id: u32, sort: u32) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(111).into();
    #[cfg(not(feature = "mock"))]
    {
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        let nav = payload::resources::nav::Nav::new(r#type, type_id, user.user_id, sort);
        let mut worker = crate::operator::WrapWorker::worker()?;
        let nav_id = worker.gen_id()?;
        crate::service::nav::AddNav::new(nav, nav_id)
            .exec()
            .await?;

        Ok(nav_id)
    }
}

/// 更新菜单(done, untested)
pub async fn update_nav(
    r#type: u8,
    type_id: u32,
    user_id: u32,
    sort: u32,
    nav_id: u32,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let nav = payload::resources::nav::Nav::new(r#type, type_id, user_id, sort);
        crate::service::nav::UpdateNav::new(nav, nav_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 删除菜单(done, untested)
pub async fn del_nav(nav_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::nav::DeleteNavReq::new(nav_id)
            .exec()
            .await?;
        Ok(())
    }
}
