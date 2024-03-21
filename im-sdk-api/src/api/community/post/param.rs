#[derive(Debug, serde::Serialize)]
pub struct CreatePostRes {
    pub(crate) post_id: u32,
}

impl CreatePostRes {
    pub async fn create_post(
        community_id: u32,
        user_id: u32,
        name: String,
        content: String,
        sort_count: i32,
    ) -> Result<Self, crate::Error> {
        let post_id = crate::handler::community::post::create_post(
            community_id,
            user_id,
            name,
            content,
            sort_count,
        )
        .await?;
        Ok(Self { post_id })
    }
}
