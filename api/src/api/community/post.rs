pub async fn post_list(
    community_id: u32,
) -> Result<Vec<payload::resources::community::post::Post>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::community::post::Post {
                user_id: 6546,
                name: "test".to_string(),
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::community::post::Post {
                user_id: 5435,
                name: "test2".to_string(),
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

pub async fn create_post(
    community_id: u32,
    user_id: u32,
    name: String,
    content: String,
) -> Result<u32, crate::Error> {
    let post = payload::resources::community::post::Post {
        community_id,
        user_id,
        name,
        content,
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(3234).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update_post(post_id: u32, name: String, content: String) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_post(id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn post_detail(
    post_id: u32,
) -> Result<payload::resources::community::post::Post, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let post = payload::resources::community::post::Post {
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
