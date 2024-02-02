pub(crate) struct CreateCommunityReq {
    community: payload::resources::community::Community,
    community_id: u32,
}

impl CreateCommunityReq {
    pub(crate) fn new(
        community: payload::resources::community::Community,
        community_id: u32,
    ) -> Self {
        Self {
            community,
            community_id,
        }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::create_community(self.community, self.community_id).await?;
        Ok(())
    }
}

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

pub(crate) struct CommunityDetailReq {
    community_id: u32,
}

impl CommunityDetailReq {
    pub(crate) fn new(community_id: u32) -> Self {
        Self { community_id }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<
        crate::operator::sqlite::query::QueryResult<crate::logic::community::CommunityDetailRes>,
        crate::SystemError,
    > {
        crate::logic::community::CommunityDetailRes::detail(self.community_id).await
    }
}

pub(crate) struct CommunityListReq {
    user_id: u32,
    page_size: u16,
    offset: u16,
}

impl CommunityListReq {
    pub(crate) fn new(user_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            user_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<
        crate::operator::sqlite::query::QueryResult<crate::logic::community::CommunityDetailRes>,
        crate::SystemError,
    > {
        crate::logic::community::CommunityDetailRes::list(self.user_id, self.page_size, self.offset)
            .await
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
