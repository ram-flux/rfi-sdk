pub async fn list() -> crate::response::Response<Vec<payload::resources::chat::Chat>> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::chat::Chat {
                chat_id: 123,
                user_id: 6546,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::chat::Chat {
                chat_id: 124,
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
