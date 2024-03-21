pub mod param;

/// 添加管理员(tested)
pub async fn add_admin(
    community_id: u32,
    r#type: u8,
    user_id: u32,
) -> crate::response::Response<param::AddAdminRes> {
    param::AddAdminRes::add_admin(community_id, r#type, user_id)
        .await
        .into()
}

/// 更新管理员类型(tested)
pub async fn update_admin(r#type: u8, admin_id: u32) -> crate::response::Response<()> {
    crate::handler::community::admin::update_admin(r#type, admin_id)
        .await
        .into()
}

/// 删除管理员(tested)
pub async fn del_admin(admin_id: u32) -> crate::response::Response<()> {
    crate::handler::community::admin::del_admin(admin_id)
        .await
        .into()
}

/// 管理员列表(tested)
pub async fn admin_list(
    community_id: u32,
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::community::admin::CommunityAdminDetailRes>> {
    crate::handler::community::admin::admin_list(community_id, page_size, offset)
        .await
        .into()
}

/// 管理员详情(tested)
pub async fn admin_detail(
    admin_id: u32,
) -> crate::response::Response<crate::logic::community::admin::CommunityAdminDetailRes> {
    crate::handler::community::admin::admin_detail(admin_id)
        .await
        .into()
}
