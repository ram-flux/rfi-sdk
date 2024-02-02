pub async fn admin_list(
    user_id: u32,
) -> Result<Vec<payload::resources::community::admin::Admin>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::community::admin::Admin {
                user_id: 6565656,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::community::admin::Admin {
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

pub async fn add_admin(community_id: u32, r#type: u8, user_id: u32) -> Result<(), crate::Error> {
    let admin = payload::resources::community::admin::Admin {
        community_id,
        r#type,
        user_id,
        created_at: payload::utils::time::now(),
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update_admin(community_id: u32, r#type: u8, user_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_admin(community_id: u32, user_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn admin_detail(
    community_id: u32,
    user_id: u32,
) -> Result<payload::resources::community::admin::Admin, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let admin = payload::resources::community::admin::Admin {
            r#type: 1,
            community_id: 43434,
            user_id: 6565656,
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
        };
        return Ok(admin).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}
