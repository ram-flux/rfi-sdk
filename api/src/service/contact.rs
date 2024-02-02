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
