/// 添加联系人(done, untested)
pub async fn add_contact(
    friend_id: u32,
    user_id: u32,
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
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        Ok(
            crate::service::contact::ContactListReq::new(user.user_id, page_size, offset)
                .exec()
                .await?,
        )
    }
}

/// 更新联系人备注(done, untested)
pub async fn update_contact_remark(contact_id: u32, remark: String) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let contact_remark = payload::resources::contact::remark::ContactRemark::new(remark);
        crate::service::contact::UpdateContactRemarkReq::new(contact_remark, contact_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 删除联系人(done, untested)
pub async fn del_contact(contact_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::contact::DeleteContactReq::new(contact_id)
            .exec()
            .await?;
        Ok(())
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
