pub async fn member_list(
    community_id: u32,
) -> crate::response::Response<Vec<payload::resources::community::member::Member>> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::community::member::Member {
                id: 123,
                user_id: 6565656,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::community::member::Member {
                id: 1243,
                user_id: 6565656,
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

pub async fn add_member(community_id: u32, user_id: u32) -> crate::response::Response<()> {
    let admin = payload::resources::community::member::Member {
        id: user_id,
        user_id,
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update_member(
    community_id: u32,
    user_id: u32,
    r#type: u8,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_member(community_id: u32, user_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn member_detail(
    community_id: u32,
    user_id: u32,
) -> crate::response::Response<payload::resources::community::member::Member> {
    #[cfg(feature = "mock")]
    {
        let member = payload::resources::community::member::Member {
            id: 123,
            r#type: 1,
            user_id: 6565656,
            name: "tester".to_string(),
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(member).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}
