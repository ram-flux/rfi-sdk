pub async fn create(
    community_id: u32,
    user_id: u32,
    name: String,
    content: String,
) -> crate::response::Response<()> {
    let admin = payload::resources::community::post::Post {
        id: 123123,
        community_id,
        user_id,
        name,
        content,
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update(id: u32, name: String, content: String) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del(id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn get(id: u32) -> crate::response::Response<payload::resources::community::post::Post> {
    #[cfg(feature = "mock")]
    {
        let post = payload::resources::community::post::Post {
            id: 123,
            user_id: 6565656,
            name: "tester".to_string(),
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
