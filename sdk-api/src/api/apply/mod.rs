//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod param;
pub mod reply;
/// 申请列表(done, untested)
pub async fn apply_list(
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::apply::ApplyDetailRes>> {
    crate::handler::apply::apply_list(page_size, offset)
        .await
        .into()
}

/// 申请详情(done, untested)
pub async fn apply_detail(
    apply_id: u32,
) -> crate::response::Response<crate::logic::apply::ApplyDetailRes> {
    crate::handler::apply::apply_detail(apply_id).await.into()
}

pub async fn create_apply(
    r#type: u8,
    type_id: u32,
    content: String,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_apply(apply_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
