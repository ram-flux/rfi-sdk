/// 成员列表(tested)
pub async fn member_list(
    community_id: u32,
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::community::member::CommunityMemberDetailRes>> {
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
        return Ok(list);
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::member::member_list(community_id, page_size, offset)
            .await
            .into()
    }
}

/// 添加成员(tested)
pub async fn add_member(
    r#type: u8,
    user_id: u32,
    community_id: u32,
    name: String,
    avatar: String,
    sort: i32,
) -> crate::response::Response<u32> {
    #[cfg(feature = "mock")]
    {
        let community_member = payload::resources::community::member::CommunityMember {
            user_id,
            ..Default::default()
        };
        return Ok(323);
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::member::add_member(
            r#type,
            user_id,
            community_id,
            name,
            avatar,
            sort,
        )
        .await
        .into()
    }
}

/// 更新成员(done, untested)
// pub async fn update_member(
//     r#type: u8,
//     community_id: u32,
//     member_id: u32,
//     name: String,
//     avatar: String,
//     sort: i32,
// ) -> crate::response::Response<()> {
//     #[cfg(feature = "mock")]
//     return Ok(());
//     #[cfg(not(feature = "mock"))]
//     {
//         let member = payload::resources::community::member::CommunityMember::new(
//             r#type,
//             community_id,
//             member_id,
//             name,
//             avatar,
//             sort,
//         );
//         crate::service::community::member::UpdateCommunityMemberReq::new(member, member_id)
//             .exec()
//             .await?;
//         Ok(())
//     }
// }

/// 更新成员类型(tested)
pub async fn update_member_type(
    r#type: u8,
    community_id: u32,
    member_id: u32,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(());
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::member::update_member_type(r#type, community_id, member_id)
            .await
            .into()
    }
}

/// 删除成员(tested)
pub async fn del_member(member_id: u32, community_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(());
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::member::del_member(member_id, community_id)
            .await
            .into()
    }
}

/// 成员详情(tested)
pub async fn member_detail(
    community_id: u32,
    user_id: u32,
) -> crate::response::Response<crate::logic::community::member::CommunityMemberDetailRes> {
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
        return Ok(member);
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::community::member::member_detail(community_id, user_id)
            .await
            .into()
    }
}
