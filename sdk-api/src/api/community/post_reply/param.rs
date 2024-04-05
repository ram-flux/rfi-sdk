//
//  Copyright 2024 Ram Flux, LLC.
//

#[derive(Debug, serde::Serialize)]
pub struct PostReplyRes {
    pub(crate) post_reply_id: u32,
}

impl PostReplyRes {
    pub async fn reply_post(
        community_id: u32,
        post_id: u32,
        user_id: u32,
        content: String,
        sort: i32,
    ) -> Result<Self, crate::Error> {
        let post_reply_id = crate::handler::community::post_reply::reply_post(
            community_id,
            post_id,
            user_id,
            content,
            sort,
        )
        .await?;
        Ok(Self { post_reply_id })
    }
}
