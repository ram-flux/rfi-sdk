/// 创建精灵(done, untested)
pub async fn create_elf(r#type: u8, name: String, avatar: String) -> crate::response::Response<u32> {
    #[cfg(feature = "mock")]
    return Ok(434).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::elf::create_elf(r#type, name, avatar)
            .await
            .into()
    }
}

/// 更新精灵(done, untested)
pub async fn update_elf(
    elf_id: u32,
    r#type: u8,
    name: String,
    avatar: String,
    status: u8,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        crate::handler::elf::update_elf(elf_id, r#type, name, avatar, status)
            .await
            .into()
    }
}

/// 精灵详情(done, untested)
pub async fn elf_detail(elf_id: u32) -> crate::response::Response<crate::logic::elf::ElfDetailRes> {
    #[cfg(feature = "mock")]
    {
        let elf = crate::logic::elf::ElfDetailRes {
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(elf).into();
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::elf::elf_detail(elf_id).await.into()
    }
}

/// 删除精灵(done, untested)
pub async fn del_elf(elf_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::elf::del_elf(elf_id).await.into()
    }
}