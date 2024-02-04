/// 管理员列表(done, untested)
pub async fn admin_list(
    community_id: u32,
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::community::admin::CommunityAdminDetailRes>, crate::Error> {
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
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::community::admin::CommunityAdminlistReq::new(
                community_id,
                page_size,
                offset,
            )
            .exec()
            .await?,
        )
    }
}

/// 添加管理员(done, untested)
pub async fn add_admin(community_id: u32, r#type: u8, user_id: u32) -> Result<(), crate::Error> {
    let admin = payload::resources::community::admin::CommunityAdmin {
        community_id,
        r#type,
        user_id,
        created_at: payload::utils::time::now(),
        updated_at: Some(payload::utils::time::now()),
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let admin = payload::resources::community::admin::CommunityAdmin::new(
            r#type,
            community_id,
            user_id,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        let admin_id = worker.gen_id()?;
        crate::service::community::admin::AddCommunityAdminReq::new(admin, admin_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 更新管理员类型(done, untested)
pub async fn update_admin(r#type: u8, admin_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let admin_type = payload::resources::community::admin::typ::CommunityAdminType::new(r#type);
        crate::service::community::admin::UpdateCommunityAdminTypeReq::new(admin_type, admin_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 删除管理员(done, untested)
pub async fn del_admin(community_id: u32, user_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::community::admin::DeleteAdminReq::new(community_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 管理员详情(done, untested)
pub async fn admin_detail(
    community_id: u32,
    user_id: u32,
) -> Result<crate::logic::community::admin::CommunityAdminDetailRes, crate::Error> {
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
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::community::admin::CommunityAdminDetailReq::new(community_id)
                .exec()
                .await?,
        )
    }
}
