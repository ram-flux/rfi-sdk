//
//  Copyright 2024 Ram Flux, LLC.
//

/// 创建精灵(done, untested)
pub async fn create_elf(r#type: u8, name: String, avatar: String) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(434);
    #[cfg(not(feature = "mock"))]
    {
        let elf = payload::resources::elf::Elf {
            r#type,
            name,
            avatar,
            status: 1,
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        let mut worker = crate::operator::WrapWorker::worker()?;
        let elf_id = worker.gen_id()?;
        crate::service::elf::CreateElfReq::new(elf, elf_id)
            .exec()
            .await?;
        Ok(elf_id)
    }
}

/// 更新精灵(done, untested)
pub async fn update_elf(
    elf_id: u32,
    r#type: u8,
    name: String,
    avatar: String,
    status: u8,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        let elf = payload::resources::elf::Elf {
            r#type,
            name,
            avatar,
            status,
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        crate::service::elf::UpdateElfReq::new(elf, elf_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 精灵详情(done, untested)
pub async fn elf_detail(elf_id: u32) -> Result<crate::logic::elf::ElfDetailRes, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let elf = crate::logic::elf::ElfDetailRes {
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(elf);
    }
    #[cfg(not(feature = "mock"))]
    {
        Ok(crate::service::elf::ElfDetailReq::new(elf_id)
            .exec()
            .await?)
    }
}

/// 删除精灵(done, untested)
pub async fn del_elf(elf_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::elf::DeleteElfReq::new(elf_id)
            .exec()
            .await?;
        Ok(())
    }
}
