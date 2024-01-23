pub async fn list() -> crate::response::Response<Vec<payload::resources::apply::Apply>> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::apply::Apply {
                id: 123,
                user_id: 6546,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::apply::Apply {
                id: 124,
                user_id: 5435,
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

pub async fn detail() -> crate::response::Response<payload::resources::apply::Apply> {
    #[cfg(feature = "mock")]
    {
        let apply = payload::resources::apply::Apply {
            id: 123,
            r#type: 1,
            user_id: 6565656,
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(apply).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn reply(apply_id: u32, user_id: u32, content: String) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del(apply_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
