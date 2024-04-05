//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) struct CreatePostReq {
    post: payload::resources::community::post::Post,
    post_id: u32,
}

impl CreatePostReq {
    pub(crate) fn new(post: payload::resources::community::post::Post, post_id: u32) -> Self {
        Self { post, post_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::create_post(self.post, self.post_id).await?;
        Ok(())
    }
}

pub(crate) struct UpdatePostReq {
    post: payload::resources::community::post::Post,
    post_id: u32,
}

impl UpdatePostReq {
    pub(crate) fn new(post: payload::resources::community::post::Post, post_id: u32) -> Self {
        Self { post, post_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_post(self.post, self.post_id).await?;
        Ok(())
    }
}

pub(crate) struct EditPostReq {
    post_info: payload::resources::community::post::info::PostInfo,
    post_id: u32,
}

impl EditPostReq {
    pub(crate) fn new(
        post_info: payload::resources::community::post::info::PostInfo,
        post_id: u32,
    ) -> Self {
        Self { post_info, post_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::edit_post(self.post_info, self.post_id).await?;
        Ok(())
    }
}

pub(crate) struct PostDetailReq {
    post_id: u32,
}

impl PostDetailReq {
    pub(crate) fn new(post_id: u32) -> Self {
        Self { post_id }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<crate::logic::community::post::PostDetailRes, crate::SystemError> {
        crate::logic::community::post::PostDetailRes::detail(self.post_id).await
    }
}

pub(crate) struct PostListReq {
    community_id: u32,
    page_size: u16,
    offset: u16,
}

impl PostListReq {
    pub(crate) fn new(community_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            community_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<Vec<crate::logic::community::post::PostDetailRes>, crate::SystemError> {
        crate::logic::community::post::PostDetailRes::list(
            self.community_id,
            self.page_size,
            self.offset,
        )
        .await
    }
}

pub(crate) struct DeletePostReq {
    post_id: u32,
}

impl DeletePostReq {
    pub(crate) fn new(post_id: u32) -> Self {
        Self { post_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_post(self.post_id).await?;
        Ok(())
    }
}
