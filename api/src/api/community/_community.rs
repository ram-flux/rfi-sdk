use crate::operator::sqlite::query::{Query as _, QueryResult};
#[cfg(not(feature = "mock"))]
use resource::Action as _;
// #[cfg(not(feature = "mock"))]

/// 社区列表
pub async fn community_list(
    user_id: u32,
    page_size: u16,
    offset: u16,
) -> Result<QueryResult<crate::logic::community::CommunityDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::community::CommunityDetailRes {
                father_id: None,
                user_id: 6546,
                name: "test".to_string(),
                status: 1,
                ..Default::default()
            },
            crate::logic::community::CommunityDetailRes {
                father_id: Some(123),
                user_id: 5435,
                name: "test2".to_string(),
                passwd: Some("asdasd".to_string()),
                pinned: true,
                status: 2,
                ..Default::default()
            },
        ];
        return Ok(QueryResult::Vec(list)).into();
    }
    #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::community::_community::CommunityListReq::new(
                user_id, page_size, offset,
            )
            .exec()
            .await?,
        )
    }
}

/// 社区详情
pub async fn community_detail(
    community_id: u32,
) -> Result<QueryResult<crate::logic::community::CommunityDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let comm = crate::logic::community::CommunityDetailRes {
            father_id: Some(123),
            user_id: 5435,
            name: "test2".to_string(),
            passwd: Some("asdasd".to_string()),
            pinned: true,
            status: 2,
            // created_at: payload::utils::time::now(),
            // updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(QueryResult::One(comm)).into();
    }
    // #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::community::_community::CommunityDetailReq::new(community_id)
                .exec()
                .await?,
        )
    }
}

/// 创建社区
pub async fn create_community(
    user_id: u32,
    father_id: Option<u32>,
    bio: String,
    name: String,
    announcement: Option<String>,
    pinned: bool,
    status: u8,
    passwd: Option<String>,
) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(3434).into();
    #[cfg(not(feature = "mock"))]
    {
        let community = payload::resources::community::Community::new(
            father_id,
            user_id,
            name,
            bio,
            passwd,
            announcement,
            pinned,
            status,
        );
        let id = payload::utils::gen_id();
        let community_action = resource::GeneralAction::Upsert {
            id: Some(id),
            resource: community,
        };

        let community_resource = crate::resources::Resources::Community(resource::Command::new(
            user_id as i64,
            community_action,
            "UpsertCommunity".to_string(),
        ));

        let pool = crate::db::USER_SQLITE_POOL.read().await;
        let pool = pool.get_pool().unwrap();
        let _ = community_resource.execute(pool.as_ref()).await;
        Ok(id).into()
    }
}

/// 更新社区
pub async fn update_community(
    community_id: u32,
    name: String,
    bio: String,
    passwd: Option<String>,
    announcement: Option<String>,
    pinned: bool,
    status: u8,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

/// 删除社区
pub async fn del_community(community_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
