pub async fn contact_list() -> Result<Vec<payload::resources::contact::Contact>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let msgs = vec![
            payload::resources::contact::Contact {
                user_id: 123123,
                friend_id: 234234,
                ..Default::default()
            },
            payload::resources::contact::Contact {
                user_id: 123123,
                friend_id: 54353,
                ..Default::default()
            },
        ];
        return Ok(msgs).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update_contact(
    user_id: u32,
    friend_id: u32,
    remark: String,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_contact(user_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn search_contact(
    keyword: String,
) -> Result<Vec<payload::resources::contact::Contact>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let msgs = vec![
            payload::resources::contact::Contact {
                user_id: 123123,
                friend_id: 234234,
                ..Default::default()
            },
            payload::resources::contact::Contact {
                user_id: 123123,
                friend_id: 54353,
                ..Default::default()
            },
        ];
        return Ok(msgs).into();
    }
    #[cfg(not(feature = "mock"))]
    todo!()
}
