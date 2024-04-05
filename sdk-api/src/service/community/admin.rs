//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) struct AddCommunityAdminReq {
    admin: payload::resources::community::admin::CommunityAdmin,
    admin_id: u32,
}

impl AddCommunityAdminReq {
    pub(crate) fn new(
        admin: payload::resources::community::admin::CommunityAdmin,
        admin_id: u32,
    ) -> Self {
        Self { admin, admin_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::add_admin(self.admin, self.admin_id).await?;
        Ok(())
    }
}

pub(crate) struct UpdateCommunityAdminTypeReq {
    admin: payload::resources::community::admin::typ::CommunityAdminType,
    admin_id: u32,
}

impl UpdateCommunityAdminTypeReq {
    pub(crate) fn new(
        admin: payload::resources::community::admin::typ::CommunityAdminType,
        admin_id: u32,
    ) -> Self {
        Self { admin, admin_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_admin_type(self.admin, self.admin_id).await?;
        Ok(())
    }
}

pub(crate) struct CommunityAdminDetailReq {
    admin_id: u32,
}

impl CommunityAdminDetailReq {
    pub(crate) fn new(admin_id: u32) -> Self {
        Self { admin_id }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<crate::logic::community::admin::CommunityAdminDetailRes, crate::SystemError> {
        crate::logic::community::admin::CommunityAdminDetailRes::detail(self.admin_id).await
    }
}

pub(crate) struct CommunityAdminlistReq {
    community_id: u32,
    page_size: u16,
    offset: u16,
}

impl CommunityAdminlistReq {
    pub(crate) fn new(community_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            community_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<Vec<crate::logic::community::admin::CommunityAdminDetailRes>, crate::SystemError>
    {
        crate::logic::community::admin::CommunityAdminDetailRes::list(
            self.community_id,
            self.page_size,
            self.offset,
        )
        .await
    }
}

pub(crate) struct DeleteAdminReq {
    admin_id: u32,
}

impl DeleteAdminReq {
    pub(crate) fn new(admin_id: u32) -> Self {
        Self { admin_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_admin(self.admin_id).await?;
        Ok(())
    }
}
