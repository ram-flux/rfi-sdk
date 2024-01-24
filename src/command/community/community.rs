pub async fn community_list(
    user_id: u32,
) -> crate::response::Response<Vec<payload::resources::community::Community>> {
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

pub async fn community_detail(
    user_id: u32,
) -> crate::response::Response<payload::resources::community::Community> {
    #[cfg(feature = "mock")]
    {
        let list = payload::resources::community::Community {
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
        };
        return Ok(list).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn create_community(
    user_id: u32,
    father_id: Option<u32>,
    bio: String,
    name: String,
    status: u8,
    passwd: Option<String>,
) -> crate::response::Response<u32> {
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
    return Ok(community.id).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update_community(
    community_id: u32,
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

pub async fn del_community(community_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}




