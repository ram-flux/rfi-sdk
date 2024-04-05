//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod param;
/// 添加菜单(done, untested)
pub async fn add_nav(
    r#type: u8,
    type_id: u32,
    sort: u32,
) -> crate::response::Response<param::AddNavRes> {
    param::AddNavRes::add_nav(r#type, type_id, sort)
        .await
        .into()
}

/// 更新菜单(done, untested)
pub async fn update_nav(
    r#type: u8,
    type_id: u32,
    user_id: u32,
    sort: u32,
    nav_id: u32,
) -> crate::response::Response<()> {
    crate::handler::nav::update_nav(r#type, type_id, user_id, sort, nav_id)
        .await
        .into()
}

/// 菜单列表(done, untested)
pub async fn nav_list(
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::nav::NavDetailRes>> {
    crate::handler::nav::nav_list(page_size, offset)
        .await
        .into()
}

/// 删除菜单(done, untested)
pub async fn del_nav(nav_id: u32) -> crate::response::Response<()> {
    crate::handler::nav::del_nav(nav_id).await.into()
}
