//
//  Copyright 2024 Ram Flux, LLC.
//

#[derive(Debug, serde::Serialize)]
pub struct CreateElfRes {
    pub(crate) elf_id: u32,
}

impl CreateElfRes {
    pub async fn create_elf(
        r#type: u8,
        name: String,
        avatar: String,
    ) -> Result<Self, crate::Error> {
        let elf_id = crate::handler::elf::create_elf(r#type, name, avatar).await?;
        Ok(Self { elf_id })
    }
}
