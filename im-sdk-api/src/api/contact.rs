/// 添加联系人(done, untested)
pub async fn add_contact(
    friend_id: u32,
    user_id: u32,
    _content: String,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        crate::handler::contact::add_contact(friend_id, user_id, _content)
            .await
            .into()
    }
}

/// 联系人列表(done, untested)
pub async fn contact_list(
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::contact::ContactDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let msgs = vec![
            crate::logic::contact::ContactDetailRes {
                user_id: 123123,
                friend_id: 234234,
                ..Default::default()
            },
            crate::logic::contact::ContactDetailRes {
                user_id: 123123,
                friend_id: 54353,
                ..Default::default()
            },
        ];
        return Ok(msgs).into();
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::contact::contact_list(page_size, offset)
            .await
            .into()
    }
}

/// 更新联系人备注(done, untested)
pub async fn update_contact_remark(contact_id: u32, remark: String) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::contact::update_contact_remark(contact_id, remark)
            .await
            .into()
    }
}

/// 删除联系人(done, untested)
pub async fn del_contact(contact_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::contact::del_contact(contact_id)
            .await
            .into()
    }
}

pub async fn search_contact(
    _keyword: String,
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
