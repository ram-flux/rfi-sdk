pub async fn favorite_list(
) -> crate::response::Response<Vec<payload::resources::favorite::Favorite>> {
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

pub async fn add_favorite(user_id: u32, content: String) -> crate::response::Response<()> {
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

pub async fn del_favorite(favorite_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn favorite_detail(
    favorite_id: u32,
) -> crate::response::Response<payload::resources::favorite::Favorite> {
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
