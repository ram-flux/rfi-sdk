pub async fn post_reply_list(
    post_id: u32,
) -> crate::response::Response<Vec<payload::resources::community::post_reply::PostReply>> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::community::post_reply::PostReply {
                id: 123,
                user_id: 6565656,
                post_id: 2343,
                content: "test".to_string(),
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::community::post_reply::PostReply {
                id: 1243,
                user_id: 6565656,
                post_id: 2343,
                content: "test".to_string(),
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
        ];
        return Ok(list).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn reply_post(
    post_id: u32,
    user_id: u32,
    content: String,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update_post_reply(post_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_post_reply(post_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn post_reply_detail(
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
