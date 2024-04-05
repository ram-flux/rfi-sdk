//
//  Copyright 2024 Ram Flux, LLC.
//

/// 成员列表(tested)
pub async fn member_list(
    community_id: u32,
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::community::member::CommunityMemberDetailRes>, crate::Error> {
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

/// 添加成员(tested)
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
            ..Default::default()
        };
        return Ok(323);
    }
    #[cfg(not(feature = "mock"))]
    {
        let mut worker = crate::operator::WrapWorker::worker()?;
        let member_id = worker.gen_id()?;
        let community_member = payload::resources::community::member::CommunityMember::new(
            member_id, r#type, name, avatar, sort,
        );

        crate::service::community::member::AddCommunityMemberReq::new(
            community_member,
            user_id,
            community_id,
        )
        .exec()
        .await?;
        Ok(member_id)
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
// ) -> Result<(), crate::Error> {
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
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(());
    #[cfg(not(feature = "mock"))]
    {
        let member_data = member_detail(community_id, member_id).await?;
        let member_type =
            payload::resources::community::member::typ::CommunityMemberType::new(r#type);
        crate::service::community::member::UpdateCommunityMemberTypeReq::new(
            member_type,
            member_data.id,
        )
        .exec()
        .await?;
        Ok(())
    }
}

/// 删除成员(tested)
pub async fn del_member(member_id: u32, community_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(());
    #[cfg(not(feature = "mock"))]
    {
        crate::service::community::member::DeleteMemberReq::new(member_id, community_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 成员详情(tested)
pub async fn member_detail(
    community_id: u32,
    user_id: u32,
) -> Result<crate::logic::community::member::CommunityMemberDetailRes, crate::Error> {
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
        Ok(
            crate::service::community::member::CommunityMemberDetailReq::new(community_id, user_id)
                .exec()
                .await?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn init_db() {
        use crate::operator::sqlite::init::DbConnection;
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let res = DbConnection::init_user_database(pri_url.to_string()).await;
        println!("init_user_database res: {res:?}");
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;
    }

    #[tokio::test]
    async fn test_member_list() {
        init_db().await;
        // Test case for member_list function
        let community_id = 3963752448;
        let page_size = 10;
        let offset = 0;

        let result = member_list(community_id, page_size, offset).await;

        println!("{:#?}", result);
        let member_list = result.unwrap();
        assert!(!member_list.is_empty());
    }

    #[tokio::test]
    async fn test_add_member() {
        init_db().await;
        crate::init_log();
        // Test case for add_member function
        let r#type = 1;
        let user_id = 1308758016;
        let community_id = 3447853056;
        let name = "Test Member".to_string();
        let avatar = "member_avatar.jpg".to_string();
        let sort = 42;

        let result = add_member(r#type, user_id, community_id, name, avatar, sort).await;

        assert!(result.is_ok());
        let member_id = result.unwrap();
        assert_ne!(member_id, 0);
    }

    #[tokio::test]
    async fn test_update_member_type() {
        init_db().await;
        // Test case for update_member function
        let r#type = 2;
        let community_id = 3963752448;
        let member_id = 1979846656;
        let _name = "Updated Member Name".to_string();
        let _avatar = "updated_avatar.jpg".to_string();
        let _sort = 55;

        let result: Result<(), crate::Error> =
            update_member_type(r#type, community_id, member_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_del_member() {
        init_db().await;
        // Test case for del_member function
        let member_id = 654446592;
        let community_id = 2294419456;

        let result: Result<(), crate::Error> = del_member(member_id, community_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_member_detail() {
        init_db().await;
        // Test case for member_detail function
        let community_id = 2420248576;
        let user_id = 654446592;

        let result: Result<
            crate::logic::community::member::CommunityMemberDetailRes,
            crate::Error,
        > = member_detail(community_id, user_id).await;

        println!("{:#?}", result);
        let _member_detail = result.unwrap();
        // Include assertions for specific details if needed
    }
}
