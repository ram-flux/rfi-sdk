//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) struct AddAccountElfReq {
    account_elf: payload::resources::account::elf::AccountElf,
    account_elf_id: u32,
}

impl AddAccountElfReq {
    pub(crate) fn new(
        account_elf: payload::resources::account::elf::AccountElf,
        account_elf_id: u32,
    ) -> Self {
        Self {
            account_elf,
            account_elf_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::create_account_elf(self.account_elf, self.account_elf_id).await?;
        Ok(())
    }
}
