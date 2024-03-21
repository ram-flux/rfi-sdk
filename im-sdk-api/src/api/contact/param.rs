#[derive(Debug, serde::Serialize)]
pub struct AddContactRes {
    pub(crate) contact_id: u32,
}

impl AddContactRes {
    /// 初始化设备(tested)
    pub async fn add_contact(friend_id: u32, user_id: u32) -> Result<Self, crate::Error> {
        let contact_id = crate::handler::contact::add_contact(friend_id, user_id).await?;
        Ok(Self { contact_id })
    }
}
