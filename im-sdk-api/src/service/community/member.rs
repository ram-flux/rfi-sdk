pub(crate) struct AddCommunityMemberReq {
    member: payload::resources::community::member::CommunityMember,
    user_id: u32,
    community_id: u32,
}

impl AddCommunityMemberReq {
    pub(crate) fn new(
        member: payload::resources::community::member::CommunityMember,
        user_id: u32,
        community_id: u32,
    ) -> Self {
        Self {
            member,
            user_id,
            community_id,
        }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::add_member(
            self.member,
            (self.user_id, self.community_id), // self.member_id
        )
        .await?;
        Ok(())
    }
}

// pub(crate) struct UpdateCommunityMemberReq {
//     member: payload::resources::community::member::CommunityMember,
//     member_id: u32,
// }

// impl UpdateCommunityMemberReq {
//     pub(crate) fn new(
//         member: payload::resources::community::member::CommunityMember,
//         member_id: u32,
//     ) -> Self {
//         Self { member, member_id }
//     }

//     pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
//         crate::logic::update::update_member(self.member, self.member_id).await?;
//         Ok(())
//     }
// }

pub(crate) struct UpdateCommunityMemberTypeReq {
    member: payload::resources::community::member::typ::CommunityMemberType,
    user_id: u32,
}

impl UpdateCommunityMemberTypeReq {
    pub(crate) fn new(
        member: payload::resources::community::member::typ::CommunityMemberType,
        user_id: u32,
    ) -> Self {
        Self { member, user_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_member_type(self.member, self.user_id).await?;
        Ok(())
    }
}

pub(crate) struct CommunityMemberDetailReq {
    community_id: u32,
    user_id: u32,
}

impl CommunityMemberDetailReq {
    pub(crate) fn new(community_id: u32, user_id: u32) -> Self {
        Self {
            community_id,
            user_id,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<crate::logic::community::member::CommunityMemberDetailRes, crate::SystemError> {
        crate::logic::community::member::CommunityMemberDetailRes::detail(
            self.community_id,
            self.user_id,
        )
        .await
    }
}

pub(crate) struct CommunityMemberlistReq {
    community_id: u32,
    page_size: u16,
    offset: u16,
}

impl CommunityMemberlistReq {
    pub(crate) fn new(community_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            community_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<Vec<crate::logic::community::member::CommunityMemberDetailRes>, crate::SystemError>
    {
        crate::logic::community::member::CommunityMemberDetailRes::list(
            self.community_id,
            self.page_size,
            self.offset,
        )
        .await
    }
}

pub(crate) struct DeleteMemberReq {
    member_id: u32,
    community_id: u32,
}

impl DeleteMemberReq {
    pub(crate) fn new(member_id: u32, community_id: u32) -> Self {
        Self {
            member_id,
            community_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_member((self.member_id, self.community_id)).await?;
        Ok(())
    }
}
