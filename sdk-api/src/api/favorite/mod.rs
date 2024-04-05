//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod param;

/// 添加收藏(done, untested)
pub async fn add_favorite(content: String) -> crate::response::Response<param::AddFavoriteRes> {
    param::AddFavoriteRes::add_favorite(content).await.into()
}

/// 删除收藏(done, untested)
pub async fn del_favorite(favorite_id: u32) -> crate::response::Response<()> {
    crate::handler::favorite::del_favorite(favorite_id)
        .await
        .into()
}

/// 收藏列表(done, untested)
pub async fn favorite_list(
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::favorite::FavoriteDetailRes>> {
    crate::handler::favorite::favorite_list(page_size, offset)
        .await
        .into()
}

/// 收藏详情(done, untested)
pub async fn favorite_detail(
    favorite_id: u32,
) -> crate::response::Response<crate::logic::favorite::FavoriteDetailRes> {
    crate::handler::favorite::favorite_detail(favorite_id)
        .await
        .into()
}
