pub async fn create(id: u32, user_id: u32, content: String) -> crate::response::Response<()> {
    let admin = payload::resources::community::post_reply::PostReply {
        id,
        user_id,
        post_id: 123213,
        content,
        created_at: payload::utils::time::now(),
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update(post_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del(post_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn get(
    post_id: u32,
) -> crate::response::Response<payload::resources::community::post_reply::PostReply> {
    #[cfg(feature = "mock")]
    {
        let post = payload::resources::community::post_reply::PostReply {
            id: 123,
            user_id: 6565656,
            post_id: 2343,
            content: "test".to_string(),
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(post).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}
