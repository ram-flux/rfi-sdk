//
//  Copyright 2024 Ram Flux, LLC.
//
#[derive(Debug, serde::Serialize)]
pub struct CreateCommunityRes {
    pub(crate) community_id: u32,
}

impl CreateCommunityRes {
    pub async fn create_community(
        father_id: Option<u32>,
        bio: String,
        name: String,
        announcement: Option<String>,
        avatar: String,
        pinned: bool,
        status: u8,
        passwd: Option<String>,
    ) -> Result<Self, crate::Error> {
        let community_id = crate::handler::community::_community::create_community(
            father_id,
            bio,
            name,
            announcement,
            avatar,
            pinned,
            status,
            passwd,
        )
        .await?;
        Ok(Self { community_id })
    }
}
