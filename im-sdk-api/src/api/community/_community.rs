use crate::service;
#[cfg(not(feature = "mock"))]

/// 创建社区(tested)
pub async fn create_community(
    father_id: Option<u32>,
    bio: String,
    name: String,
    announcement: Option<String>,
    avatar: String,
    pinned: bool,
    status: u8,
    passwd: Option<String>,
) -> crate::response::Response<u32> {
    #[cfg(feature = "mock")]
    return Ok(3434);
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::_community::create_community(
            father_id,
            bio,
            name,
            announcement,
            avatar,
            pinned,
            status,
            passwd,
        )
        .await
        .into()
    }
}

/// 更新社区(tested)
pub async fn update_community(
    community_id: u32,
    name: String,
    bio: String,
    passwd: Option<String>,
    announcement: Option<String>,
    avatar: String,
    pinned: bool,
    status: u8,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(());
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::_community::update_community(
            community_id,
            name,
            bio,
            passwd,
            announcement,
            avatar,
            pinned,
            status,
        )
        .await
        .into()
    }
}

/// 删除社区(tested)
pub async fn del_community(community_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(());
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::_community::del_community(community_id)
            .await
            .into()
    }
}

/// 社区列表(tested)
pub async fn community_list(
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::community::_community::CommunityDetailRes>> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::community::_community::CommunityDetailRes {
                father_id: None,
                user_id: 6546,
                name: "test".to_string(),
                status: 1,
                ..Default::default()
            },
            crate::logic::community::_community::CommunityDetailRes {
                father_id: Some(123),
                user_id: 5435,
                name: "test2".to_string(),
                passwd: Some("asdasd".to_string()),
                pinned: true,
                status: 2,
                ..Default::default()
            },
        ];
        return Ok(list);
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::_community::community_list(page_size, offset)
            .await
            .into()
    }
}

/// 社区详情(tested)
pub async fn community_detail(
    community_id: u32,
) -> crate::response::Response<crate::logic::community::_community::CommunityDetailRes> {
    #[cfg(feature = "mock")]
    {
        let comm = crate::logic::community::_community::CommunityDetailRes {
            father_id: Some(123),
            user_id: 5435,
            name: "test2".to_string(),
            passwd: Some("asdasd".to_string()),
            pinned: true,
            status: 2,
            ..Default::default()
        };
        return Ok(comm);
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::_community::community_detail(community_id)
            .await
            .into()
    }
}
