pub mod param;
/// 添加联系人(done, untested)
pub async fn add_contact(
    friend_id: u32,
    user_id: u32,
) -> crate::response::Response<param::AddContactRes> {
    param::AddContactRes::add_contact(friend_id, user_id)
        .await
        .into()
}

/// 联系人列表(done, untested)
pub async fn contact_list(
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::contact::ContactDetailRes>> {
    crate::handler::contact::contact_list(page_size, offset)
        .await
        .into()
}

/// 更新联系人备注(done, untested)
pub async fn update_contact_remark(
    contact_id: u32,
    remark: String,
) -> crate::response::Response<()> {
    crate::handler::contact::update_contact_remark(contact_id, remark)
        .await
        .into()
}

/// 删除联系人(done, untested)
pub async fn del_contact(contact_id: u32) -> crate::response::Response<()> {
    crate::handler::contact::del_contact(contact_id)
        .await
        .into()
}

pub async fn search_contact(
    _keyword: String,
) -> crate::response::Response<Vec<payload::resources::contact::Contact>> {
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
