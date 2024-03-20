/// 菜单列表(done, untested)
pub async fn nav_list(
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::nav::NavDetailRes>> {
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
        crate::handler::nav::nav_list(page_size, offset)
            .await
            .into()
    }
}

/// 添加菜单(done, untested)
pub async fn add_nav(r#type: u8, type_id: u32, sort: u32) -> crate::response::Response<u32> {
    #[cfg(feature = "mock")]
    return Ok(111).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::nav::add_nav(r#type, type_id, sort)
            .await
            .into()
    }
}

/// 更新菜单(done, untested)
pub async fn update_nav(
    r#type: u8,
    type_id: u32,
    user_id: u32,
    sort: u32,
    nav_id: u32,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::nav::update_nav(r#type, type_id, user_id, sort, nav_id)
            .await
            .into()
    }
}

/// 删除菜单(done, untested)
pub async fn del_nav(nav_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::nav::del_nav(nav_id).await.into()
    }
}
