pub(crate) struct InitDeviceReq {
    device: payload::resources::device::Device,
    device_id: u32,
    account: payload::resources::account::Account,
    account_id: u32, // recv_list: Vec<u32>,
}

impl InitDeviceReq {
    pub(crate) fn new(
        device: payload::resources::device::Device,
        device_id: u32,
        account: payload::resources::account::Account,
        account_id: u32,
    ) -> Self {
        Self {
            device,
            device_id,
            account,
            account_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        // crate::logic::device::new_device(self.device, self.device_id).await?;
        crate::logic::upsert::new_device(self.device, self.device_id).await?;
        // upsert_resource!(new_account, account, account_action, "UpsertAccount");
        // super::account::new_account(self.account, self.account_id).await?;
        crate::logic::upsert::new_account(self.account, self.account_id).await?;
        Ok(())
    }
}

pub(crate) struct UpdateTokenReq {
    token: payload::resources::device::token::Token,
    device_id: u32,
}

impl UpdateTokenReq {
    pub(crate) fn new(token: payload::resources::device::token::Token, device_id: u32) -> Self {
        Self { token, device_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_token(self.token, self.device_id).await?;
        Ok(())
    }
}

pub(crate) struct DelDeviceReq {
    device_id: u32,
}

impl DelDeviceReq {
    pub(crate) fn new(device_id: u32) -> Self {
        Self { device_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_device(self.device_id).await?;
        Ok(())
    }
}
