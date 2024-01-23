pub async fn create(
    user_id: u32,
    father_id: Option<u32>,
    name: String,
    status: u8,
    passwd: Option<String>,
) -> crate::response::Response<()> {
    let community = payload::resources::community::Community {
        id: 12324,
        father_id,
        user_id,
        name,
        passwd,
        status,
        created_at: payload::utils::time::now(),
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn list() -> crate::response::Response<Vec<payload::resources::community::Community>> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::community::Community {
                id: 123,
                father_id: None,
                user_id: 6546,
                name: "test".to_string(),
                status: 1,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::community::Community {
                id: 124,
                father_id: Some(123),
                user_id: 5435,
                name: "test2".to_string(),
                passwd: Some("asdasd".to_string()),
                pinned: true,
                status: 2,
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

pub async fn update(
    id: u32,
    name: String,
    bio: String,
    passwd: Option<String>,
    announcement: Option<String>,
    pinned: bool,
    status: u8,
) -> crate::response::Response<()> {
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

pub async fn create_child(
    user_id: u32,
    father_id: Option<u32>,
    name: String,
    status: u8,
    passwd: Option<String>,
) -> crate::response::Response<()> {
    let community = payload::resources::community::Community {
        id: 12324,
        father_id,
        user_id,
        name,
        passwd,
        status,
        created_at: payload::utils::time::now(),
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update_child(
    id: u32,
    name: String,
    bio: String,
    passwd: Option<String>,
    announcement: Option<String>,
    pinned: bool,
    status: u8,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_child(id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn get_post_list(
    community_id: u32,
) -> crate::response::Response<Vec<payload::resources::community::post::Post>> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::community::post::Post {
                id: community_id,
                user_id: 6546,
                name: "test".to_string(),
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::community::post::Post {
                id: community_id,
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
) -> crate::response::Response<()> {
    let post = payload::resources::community::post::Post {
        id: 234324,
        community_id,
        user_id,
        name,
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

pub async fn get_post_detail(
    post_id: u32,
) -> crate::response::Response<payload::resources::community::post::Post> {
    #[cfg(feature = "mock")]
    return Ok(payload::resources::community::post::Post {
        id: 234324,
        community_id: 234324,
        user_id: 43535,
        name: "test".to_string(),
        content: "test".to_string(),
        created_at: payload::utils::time::now(),
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    })
    .into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn reply_post(post_id: u32, content: String) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
