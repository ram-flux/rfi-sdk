pub async fn nav_list() -> crate::response::Response<Vec<payload::resources::nav::Nav>> {
    #[cfg(feature = "mock")]
    {
        let msgs = vec![
            payload::resources::nav::Nav {
                user_id: 123123,
                ..Default::default()
            },
            payload::resources::nav::Nav {
                user_id: 123123,
                ..Default::default()
            },
        ];
        return Ok(msgs).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn add_nav(user_id: u32, r#type: u8, type_id: u32, sort: u8) -> Result<(), crate::Error> {
    let nav = payload::resources::nav::Nav {
        user_id,
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update_nav(nav_id: u32, sort: u8) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_nav(nav_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
