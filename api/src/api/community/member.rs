/// 成员列表(done, untested)
pub async fn member_list(
    community_id: u32,
    page_size: u16,
    offset: u16,
) -> Result<
    crate::operator::sqlite::query::QueryResult<
        crate::logic::community::member::CommunityMemberDetailRes,
    >,
    crate::Error,
> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::community::member::CommunityMemberDetailRes {
                user_id: 6565656,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            crate::logic::community::member::CommunityMemberDetailRes {
                user_id: 6565656,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
        ];
        return Ok(crate::operator::sqlite::query::QueryResult::Vec(list));
    }
    #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::community::member::CommunityMemberlistReq::new(
                community_id,
                page_size,
                offset,
            )
            .exec()
            .await?,
        )
    }
}

/// 添加成员类型(done, untested)
pub async fn add_member(
    r#type: u8,
    user_id: u32,
    community_id: u32,
    name: String,
    avatar: String,
    sort: i32,
) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let community_member = payload::resources::community::member::CommunityMember {
            user_id,
            ..Default::default()
        };
    }
    return Ok(323).into();
    #[cfg(not(feature = "mock"))]
    {
        let community_member = payload::resources::community::member::CommunityMember::new(
            r#type,
            community_id,
            user_id,
            name,
            avatar,
            sort,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        let member_id = worker.gen_id()?;
        crate::service::community::member::AddCommunityMemberReq::new(community_member, member_id)
            .exec()
            .await?;
        Ok(member_id)
    }
}

/// 更新成员(done, untested)
pub async fn update_member(
    r#type: u8,
    community_id: u32,
    member_id: u32,
    name: String,
    avatar: String,
    sort: i32,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let member = payload::resources::community::member::CommunityMember::new(
            r#type,
            community_id,
            member_id,
            name,
            avatar,
            sort,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        crate::service::community::member::UpdateCommunityMemberReq::new(member, member_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 删除成员(done, untested)
pub async fn del_member(member_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::community::member::DeleteMemberReq::new(member_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 成员详情(done, untested)
pub async fn member_detail(
    community_id: u32,
    user_id: u32,
) -> Result<
    crate::operator::sqlite::query::QueryResult<
        crate::logic::community::member::CommunityMemberDetailRes,
    >,
    crate::Error,
> {
    #[cfg(feature = "mock")]
    {
        let member = crate::logic::community::member::CommunityMemberDetailRes {
            r#type: 1,
            user_id: 6565656,
            name: "tester".to_string(),
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(crate::operator::sqlite::query::QueryResult::One(member));
    }
    #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::community::member::CommunityMemberDetailReq::new(community_id)
                .exec()
                .await?,
        )
    }
}
