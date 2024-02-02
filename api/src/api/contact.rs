/// 添加联系人(done, untested)
pub async fn add_contact(
    friend_id: u32,
    user_id: u32,
    content: String,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        let mut worker = crate::operator::WrapWorker::worker()?;
        let trace_id = worker.gen_id()?;
        let contact = payload::resources::contact::Contact::new(user_id, friend_id);
        crate::service::contact::AddContactReq::new(contact, trace_id)
            .exec()
            .await?;
        Ok(())
    }
}

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
