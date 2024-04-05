//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod param;
/// 创建精灵(done, untested)
pub async fn create_elf(
    r#type: u8,
    name: String,
    avatar: String,
) -> crate::response::Response<param::CreateElfRes> {
    param::CreateElfRes::create_elf(r#type, name, avatar)
        .await
        .into()
}

/// 更新精灵(done, untested)
pub async fn update_elf(
    elf_id: u32,
    r#type: u8,
    name: String,
    avatar: String,
    status: u8,
) -> crate::response::Response<()> {
    crate::handler::elf::update_elf(elf_id, r#type, name, avatar, status)
        .await
        .into()
}

/// 精灵详情(done, untested)
pub async fn elf_detail(elf_id: u32) -> crate::response::Response<crate::logic::elf::ElfDetailRes> {
    crate::handler::elf::elf_detail(elf_id).await.into()
}

/// 删除精灵(done, untested)
pub async fn del_elf(elf_id: u32) -> crate::response::Response<()> {
    crate::handler::elf::del_elf(elf_id).await.into()
}
