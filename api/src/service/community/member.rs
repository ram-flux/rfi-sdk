pub(crate) struct AddCommunityMemberReq {
    member: payload::resources::community::member::CommunityMember,
    member_id: u32,
}

impl AddCommunityMemberReq {
    pub(crate) fn new(
        member: payload::resources::community::member::CommunityMember,
        member_id: u32,
    ) -> Self {
        Self { member, member_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::add_member(self.member, self.member_id).await?;
        Ok(())
    }
}

pub(crate) struct UpdateCommunityMemberReq {
    member: payload::resources::community::member::CommunityMember,
    member_id: u32,
}

impl UpdateCommunityMemberReq {
    pub(crate) fn new(
        member: payload::resources::community::member::CommunityMember,
        member_id: u32,
    ) -> Self {
        Self { member, member_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_member(self.member, self.member_id).await?;
        Ok(())
    }
}

pub(crate) struct CommunityMemberDetailReq {
    member_id: u32,
}

impl CommunityMemberDetailReq {
    pub(crate) fn new(member_id: u32) -> Self {
        Self { member_id }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<crate::logic::community::member::CommunityMemberDetailRes, crate::SystemError> {
        crate::logic::community::member::CommunityMemberDetailRes::detail(self.member_id).await
    }
}

pub(crate) struct CommunityMemberlistReq {
    member_id: u32,
    page_size: u16,
    offset: u16,
}

impl CommunityMemberlistReq {
    pub(crate) fn new(member_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            member_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<Vec<crate::logic::community::member::CommunityMemberDetailRes>, crate::SystemError>
    {
        crate::logic::community::member::CommunityMemberDetailRes::list(
            self.member_id,
            self.page_size,
            self.offset,
        )
        .await
    }
}

pub(crate) struct DeleteMemberReq {
    member_id: u32,
}

impl DeleteMemberReq {
    pub(crate) fn new(member_id: u32) -> Self {
        Self { member_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_member(self.member_id).await?;
        Ok(())
    }
}
