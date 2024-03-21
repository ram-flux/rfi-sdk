#[derive(Debug, serde::Serialize)]
pub struct ReplyApplyRes {
    pub(crate) reply_apply_id: u32,
}

impl ReplyApplyRes {
    pub async fn reply_apply(
        apply_id: u32,
        content: String,
        status: u8,
    ) -> Result<Self, crate::Error> {
        let reply_apply_id =
            crate::handler::apply::reply::reply_apply(apply_id, content, status).await?;
        Ok(Self { reply_apply_id })
    }
}
