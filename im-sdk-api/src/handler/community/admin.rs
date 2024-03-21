/// 添加管理员(tested)
pub async fn add_admin(community_id: u32, r#type: u8, user_id: u32) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(111).into();
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
        Ok(admin_id)
    }
}

/// 更新管理员类型(tested)
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

/// 删除管理员(tested)
pub async fn del_admin(admin_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::community::admin::DeleteAdminReq::new(admin_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 管理员列表(tested)
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

/// 管理员详情(tested)
pub async fn admin_detail(
    admin_id: u32,
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
        Ok(
            crate::service::community::admin::CommunityAdminDetailReq::new(admin_id)
                .exec()
                .await?,
        )
    }
}

#[cfg(test)]
mod test {

    async fn init_db() {
        use crate::operator::sqlite::init::DbConnection;
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let res = DbConnection::init_user_database(pri_url.to_string()).await;
        println!("init_user_database res: {res:?}");
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;
    }

    #[tokio::test]
    async fn test_add_admin() {
        init_db().await;

        let community_id = 662835200;
        let r#type = 1;
        let user_id = 121769984;
        let res = crate::api::community::admin::add_admin(community_id, r#type, user_id).await;
        println!("res: {res:#?}");
    }

    #[tokio::test]
    async fn test_update_admin() {
        init_db().await;

        let r#type = 2;
        let admin_id = 1799491584;
        let res = crate::api::community::admin::update_admin(r#type, admin_id).await;
        println!("res: {res:#?}");
    }

    #[tokio::test]
    async fn test_del_admin() {
        init_db().await;

        let admin_id = 1799491584;
        let res = crate::api::community::admin::del_admin(admin_id).await;
        println!("res: {res:#?}");
    }

    #[tokio::test]
    async fn test_admin_list() {
        init_db().await;

        let community_id = 662835200;
        let page_size = 3;
        let offset = 0;
        let res = crate::api::community::admin::admin_list(community_id, page_size, offset).await;
        println!("res: {res:#?}");
    }

    #[tokio::test]
    async fn test_admin_detail() {
        init_db().await;

        let admin_id = 511840256;
        let res = crate::api::community::admin::admin_detail(admin_id).await;
        println!("res: {res:#?}");
    }
}
