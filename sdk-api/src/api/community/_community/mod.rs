//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod param;
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
) -> crate::response::Response<param::CreateCommunityRes> {
    param::CreateCommunityRes::create_community(
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

/// 删除社区(tested)
pub async fn del_community(community_id: u32) -> crate::response::Response<()> {
    crate::handler::community::_community::del_community(community_id)
        .await
        .into()
}

/// 社区列表(tested)
pub async fn community_list(
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::community::_community::CommunityDetailRes>> {
    crate::handler::community::_community::community_list(page_size, offset)
        .await
        .into()
}

/// 社区详情(tested)
pub async fn community_detail(
    community_id: u32,
) -> crate::response::Response<crate::logic::community::_community::CommunityDetailRes> {
    crate::handler::community::_community::community_detail(community_id)
        .await
        .into()
}
