pub async fn chat_list(
    user_id: u32,
) -> crate::response::Response<Vec<payload::resources::chat::Chat>> {
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

pub async fn search_chat(
    chat_id: u32,
    keyword: String,
) -> crate::response::Response<Vec<payload::resources::message::Message>> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::message::Message {
                chat_id: 123,
                user_id: 6546,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::message::Message {
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
