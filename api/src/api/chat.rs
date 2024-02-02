pub async fn chat_list(user_id: u32) -> Result<Vec<payload::resources::chat::Chat>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::chat::Chat {
                user_id: 6546,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            payload::resources::chat::Chat {
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
) -> Result<Vec<payload::resources::message::Message>, crate::Error> {
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

pub async fn pin_chat(chat: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
