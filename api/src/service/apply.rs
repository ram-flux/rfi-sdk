pub(crate) struct ReplyApplyReq {
    account: payload::resources::account::Account,
    account_id: u32,
}

impl ReplyApplyReq {
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

pub(crate) struct UpdateApplyReq {
    avatar: payload::resources::account::avatar::Avatar,
    account_id: u32,
}

impl UpdateApplyReq {
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

pub(crate) struct ApplyDetailReq {
    admin_id: u32,
}

impl ApplyDetailReq {
    pub(crate) fn new(admin_id: u32) -> Self {
        Self { admin_id }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<
        crate::operator::sqlite::query::QueryResult<crate::logic::apply::ApplyDetailRes>,
        crate::SystemError,
    > {
        crate::logic::apply::ApplyDetailRes::detail(self.admin_id).await
    }
}

pub(crate) struct ApplyListReq {
    r#type: u8,
    type_id: u32,
    page_size: u16,
    offset: u16,
}

impl ApplyListReq {
    pub(crate) fn new(r#type: u8, type_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            r#type,
            type_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<
        crate::operator::sqlite::query::QueryResult<crate::logic::apply::ApplyDetailRes>,
        crate::SystemError,
    > {
        crate::logic::apply::ApplyDetailRes::list(
            self.r#type,
            self.type_id,
            self.page_size,
            self.offset,
        )
        .await
    }
}

pub(crate) struct DeleteApplyReq {
    apply_id: u32,
}

impl DeleteApplyReq {
    pub(crate) fn new(apply_id: u32) -> Self {
        Self { apply_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_apply(self.apply_id).await?;
        Ok(())
    }
}

pub(crate) struct DeleteApplyReplyReq {
    apply_id: u32,
}

impl DeleteApplyReplyReq {
    pub(crate) fn new(apply_id: u32) -> Self {
        Self { apply_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_apply_reply(self.apply_id).await?;
        Ok(())
    }
}
