pub(crate) struct UpdateAccountReq {
    account: payload::resources::account::Account,
    account_id: u32,
}

impl UpdateAccountReq {
    pub(crate) fn new(account: payload::resources::account::Account, account_id: u32) -> Self {
        Self {
            account,
            account_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::new_account(self.account, self.account_id).await?;
        Ok(())
    }
}

pub(crate) struct UpdateAvatarReq {
    avatar: payload::resources::account::avatar::Avatar,
    account_id: u32,
}

impl UpdateAvatarReq {
    pub(crate) fn new(
        avatar: payload::resources::account::avatar::Avatar,
        account_id: u32,
    ) -> Self {
        Self { avatar, account_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_avatar(self.avatar, self.account_id).await?;
        Ok(())
    }
}

pub(crate) struct AccountDetailReq {
    user_id: u32,
}

impl AccountDetailReq {
    pub(crate) fn new(user_id: u32) -> Self {
        Self { user_id }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<
        crate::operator::sqlite::query::QueryResult<crate::logic::account::AccountDetailRes>,
        crate::SystemError,
    > {
        crate::logic::account::AccountDetailRes::exec(self.user_id).await
    }
}
