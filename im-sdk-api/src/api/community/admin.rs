/// 添加管理员(tested)
pub async fn add_admin(
    community_id: u32,
    r#type: u8,
    user_id: u32,
) -> crate::response::Response<u32> {
    #[cfg(feature = "mock")]
    return Ok(111).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::admin::add_admin(community_id, r#type, user_id)
            .await
            .into()
    }
}

/// 更新管理员类型(tested)
pub async fn update_admin(r#type: u8, admin_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::admin::update_admin(r#type, admin_id)
            .await
            .into()
    }
}

/// 删除管理员(tested)
pub async fn del_admin(admin_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::admin::del_admin(admin_id)
            .await
            .into()
    }
}

/// 管理员列表(tested)
pub async fn admin_list(
    community_id: u32,
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::community::admin::CommunityAdminDetailRes>> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::community::admin::CommunityAdminDetailRes {
                user_id: 6565656,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            crate::logic::community::admin::CommunityAdminDetailRes {
                user_id: 6565656,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
        ];
        return Ok(list);
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::admin::admin_list(community_id, page_size, offset)
            .await
            .into()
    }
}

/// 管理员详情(tested)
pub async fn admin_detail(
    admin_id: u32,
) -> crate::response::Response<crate::logic::community::admin::CommunityAdminDetailRes> {
    #[cfg(feature = "mock")]
    {
        let admin = crate::logic::community::admin::CommunityAdminDetailRes {
            r#type: 1,
            community_id: 43434,
            user_id: 6565656,
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(admin);
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::admin::admin_detail(admin_id)
            .await
            .into()
    }
}
