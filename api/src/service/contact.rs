pub(crate) struct AddContactReq {
    contact: payload::resources::contact::Contact,
    contact_id: u32,
}

impl AddContactReq {
    pub(crate) fn new(contact: payload::resources::contact::Contact, contact_id: u32) -> Self {
        Self {
            contact,
            contact_id,
        }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::add_contact(self.contact, self.contact_id).await?;
        Ok(())
    }
}

pub(crate) struct UpdateContactRemarkReq {
    contact_remark: payload::resources::contact::remark::ContactRemark,
    contact_id: u32,
}

impl UpdateContactRemarkReq {
    pub(crate) fn new(
        contact_remark: payload::resources::contact::remark::ContactRemark,
        contact_id: u32,
    ) -> Self {
        Self {
            contact_remark,
            contact_id,
        }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_contact_remark(self.contact_remark, self.contact_id).await?;
        Ok(())
    }
}

pub(crate) struct ContactDetailReq {
    contact_id: u32,
}

impl ContactDetailReq {
    pub(crate) fn new(post_id: u32) -> Self {
        Self {
            contact_id: post_id,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<crate::logic::contact::ContactDetailRes, crate::SystemError> {
        crate::logic::contact::ContactDetailRes::detail(self.contact_id).await
    }
}

pub(crate) struct ContactListReq {
    user_id: u32,
    page_size: u16,
    offset: u16,
}

impl ContactListReq {
    pub(crate) fn new(user_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            user_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<Vec<crate::logic::contact::ContactDetailRes>, crate::SystemError> {
        crate::logic::contact::ContactDetailRes::list(self.user_id, self.page_size, self.offset)
            .await
    }
}

pub(crate) struct DeleteContactReq {
    contact_id: u32,
}

impl DeleteContactReq {
    pub(crate) fn new(contact_id: u32) -> Self {
        Self { contact_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_contact(self.contact_id).await?;
        Ok(())
    }
}
