pub async fn apply_list(
    user_id: u32,
) -> Result<Vec<payload::resources::apply::Apply>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::apply::Apply {
                user_id: 6546,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::apply::Apply {
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

pub async fn apply_detail(apply_id: u32) -> Result<payload::resources::apply::Apply, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let apply = payload::resources::apply::Apply {
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

pub async fn reply_apply(
    apply_id: u32,
    user_id: u32,
    content: String,
    status: u8,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_apply(apply_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
