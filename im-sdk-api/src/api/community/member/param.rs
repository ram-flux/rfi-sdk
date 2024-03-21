#[derive(Debug, serde::Serialize)]
pub struct AddMemberRes {
    pub(crate) member_id: u32,
}

impl AddMemberRes {
    pub async fn add_member(
        r#type: u8,
        user_id: u32,
        community_id: u32,
        name: String,
        avatar: String,
        sort: i32,
    ) -> Result<Self, crate::Error> {
        let member_id = crate::handler::community::member::add_member(
            r#type,
            user_id,
            community_id,
            name,
            avatar,
            sort,
        )
        .await?;
        Ok(Self { member_id })
    }
}
