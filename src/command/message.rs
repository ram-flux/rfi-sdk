pub async fn push(
    content: String,
    mode: u8,
    from_id: i32,
    user_id: i32,
    chat_type: u8,
) -> crate::response::Response<()> {
    let mut message = payload::resources::message::Message::default();
    message.datas = content;
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn pull(
    message_id: u32,
) -> crate::response::Response<Vec<payload::resources::message::Message>> {
    #[cfg(feature = "mock")]
    {
        let msgs = vec![
            payload::resources::message::Message {
                datas: "asdsad".to_string(),
                mode: 1,
                ..Default::default()
            },
            payload::resources::message::Message {
                datas: "fsdfs".to_string(),
                mode: 1,
                ..Default::default()
            },
        ];
        return Ok(msgs).into();
    }

    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update(
    message_id: u32,
    content: String,
) -> crate::response::Response<payload::resources::message::Message> {
    let mut message = payload::resources::message::Message::default();
    message.datas = content;
    #[cfg(feature = "mock")]
    return Ok(message).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del(id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
