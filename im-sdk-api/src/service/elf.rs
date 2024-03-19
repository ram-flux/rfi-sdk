pub(crate) struct CreateElfReq {
    elf: payload::resources::elf::Elf,
    elf_id: u32,
}

impl CreateElfReq {
    pub(crate) fn new(elf: payload::resources::elf::Elf, elf_id: u32) -> Self {
        Self { elf, elf_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::create_elf(self.elf, self.elf_id).await?;
        Ok(())
    }
}

pub(crate) struct UpdateElfReq {
    elf: payload::resources::elf::Elf,
    elf_id: u32,
}

impl UpdateElfReq {
    pub(crate) fn new(elf: payload::resources::elf::Elf, elf_id: u32) -> Self {
        Self { elf, elf_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_elf(self.elf, self.elf_id).await?;
        Ok(())
    }
}

pub(crate) struct ElfDetailReq {
    elf_id: u32,
}

impl ElfDetailReq {
    pub(crate) fn new(elf_id: u32) -> Self {
        Self { elf_id }
    }
    pub(crate) async fn exec(self) -> Result<crate::logic::elf::ElfDetailRes, crate::SystemError> {
        crate::logic::elf::ElfDetailRes::detail(self.elf_id).await
    }
}

// pub(crate) struct ElfListReq {
//     user_id: u32,
//     page_size: u16,
//     offset: u16,
// }

// impl ElfListReq {
//     pub(crate) fn new(user_id: u32, page_size: u16, offset: u16) -> Self {
//         Self {
//             user_id,
//             page_size,
//             offset,
//         }
//     }
//     pub(crate) async fn exec(
//         self,
//     ) -> Result<Vec<crate::logic::chat::ChatDetailRes>, crate::SystemError> {
//         crate::logic::chat::ChatDetailRes::list(self.user_id, self.page_size, self.offset).await
//     }
// }

pub(crate) struct DeleteElfReq {
    elf_id: u32,
}

impl DeleteElfReq {
    pub(crate) fn new(elf_id: u32) -> Self {
        Self { elf_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_elf(self.elf_id).await?;
        Ok(())
    }
}
