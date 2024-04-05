//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) struct CreateCommunityReq {
    community: payload::resources::community::Community,
    community_id: u32,
    community_member: payload::resources::community::member::CommunityMember,
    user_id: u32,
    community_admin: payload::resources::community::admin::CommunityAdmin,
    community_admin_id: u32,
}

impl CreateCommunityReq {
    pub(crate) fn new(
        community: payload::resources::community::Community,
        community_id: u32,
        community_member: payload::resources::community::member::CommunityMember,
        user_id: u32,
        community_admin: payload::resources::community::admin::CommunityAdmin,
        community_admin_id: u32,
    ) -> Self {
        Self {
            community,
            community_id,
            community_member,
            user_id,
            community_admin,
            community_admin_id,
        }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::create_community(self.community, self.community_id).await?;
        crate::logic::upsert::add_member(
            self.community_member,
            (self.user_id, self.community_id),
            // self.community_member_id
        )
        .await?;
        crate::logic::upsert::add_admin(self.community_admin, self.community_admin_id).await?;
        Ok(())
    }
}

pub(crate) struct UpdateCommunityReq {
    community: payload::resources::community::info::CommunityInfo,
    community_id: u32,
}

impl UpdateCommunityReq {
    pub(crate) fn new(
        community: payload::resources::community::info::CommunityInfo,
        community_id: u32,
    ) -> Self {
        Self {
            community,
            community_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_community(self.community, self.community_id).await?;
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

pub(crate) struct CommunityDetailReq {
    community_id: u32,
}

impl CommunityDetailReq {
    pub(crate) fn new(community_id: u32) -> Self {
        Self { community_id }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<crate::logic::community::_community::CommunityDetailRes, crate::SystemError> {
        crate::logic::community::_community::CommunityDetailRes::detail(self.community_id).await
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
    ) -> Result<Vec<crate::logic::community::_community::CommunityDetailRes>, crate::SystemError>
    {
        crate::logic::community::_community::CommunityDetailRes::list(
            self.user_id,
            self.page_size,
            self.offset,
        )
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

pub(crate) struct DeleteCommunityReq {
    community_id: u32,
}

impl DeleteCommunityReq {
    pub(crate) fn new(community_id: u32) -> Self {
        Self { community_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_community(self.community_id).await?;
        Ok(())
    }
}
