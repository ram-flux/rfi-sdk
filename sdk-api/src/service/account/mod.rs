//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) mod elf;
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
    avatar: payload::resources::account::avatar::AccountAvatar,
    account_id: u32,
}

impl UpdateAvatarReq {
    pub(crate) fn new(
        avatar: payload::resources::account::avatar::AccountAvatar,
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
    ) -> Result<crate::logic::account::AccountDetailRes, crate::SystemError> {
        crate::logic::account::AccountDetailRes::detail(self.user_id).await
    }
}

pub(crate) struct AccountListReq {
    page_size: u16,
    offset: u16,
}

impl AccountListReq {
    pub(crate) fn new(page_size: u16, offset: u16) -> Self {
        Self { page_size, offset }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<Vec<crate::logic::account::AccountDetailRes>, crate::SystemError> {
        crate::logic::account::AccountDetailRes::list(self.page_size, self.offset).await
    }
}

pub(crate) struct AddCommunityReq {
    community: payload::resources::account::community::AccountCommunity,
    community_id: u32,
}

impl AddCommunityReq {
    pub(crate) fn new(
        community: payload::resources::account::community::AccountCommunity,
        community_id: u32,
    ) -> Self {
        Self {
            community,
            community_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::join_community(self.community, self.community_id).await?;
        Ok(())
    }
}

pub(crate) struct QuitCommunityReq {
    community_id: u32,
}

impl QuitCommunityReq {
    pub(crate) fn new(community_id: u32) -> Self {
        Self { community_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::quit_community(self.community_id).await?;
        Ok(())
    }
}
