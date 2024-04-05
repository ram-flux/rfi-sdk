//
//  Copyright 2024 Ram Flux, LLC.
//

#[derive(Debug, serde::Serialize)]
pub struct AddAdminRes {
    pub(crate) admin_id: u32,
}

impl AddAdminRes {
    pub async fn add_admin(
        community_id: u32,
        r#type: u8,
        user_id: u32,
    ) -> Result<Self, crate::Error> {
        let admin_id =
            crate::handler::community::admin::add_admin(community_id, r#type, user_id).await?;
        Ok(Self { admin_id })
    }
}
