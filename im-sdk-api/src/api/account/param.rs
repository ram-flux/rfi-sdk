#[derive(Debug, serde::Serialize)]
pub struct CreateAccountElfRes {
    pub(crate) account_elf_id: u32,
}

impl CreateAccountElfRes {
    /// 初始化设备(tested)
    pub async fn create_account_elf(name: String, avatar: String) -> Result<Self, crate::Error> {
        let account_elf_id = crate::handler::account::create_account_elf(name, avatar).await?;
        Ok(Self { account_elf_id })
    }
}
