pub async fn favorite_list() -> Result<Vec<payload::resources::favorite::Favorite>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let msgs = vec![
            payload::resources::favorite::Favorite {
                user_id: 123123,
                ..Default::default()
            },
            payload::resources::favorite::Favorite {
                user_id: 123123,
                ..Default::default()
            },
        ];
        return Ok(msgs).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn add_favorite(user_id: u32, content: String) -> Result<(), crate::Error> {
    let favorite = payload::resources::favorite::Favorite {
        user_id,
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_favorite(favorite_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn favorite_detail(
    favorite_id: u32,
) -> Result<payload::resources::favorite::Favorite, crate::Error> {
    let favorite = payload::resources::favorite::Favorite {
        id: favorite_id,
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(favorite).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
