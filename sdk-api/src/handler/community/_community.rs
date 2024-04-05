//
//  Copyright 2024 Ram Flux, LLC.
//

use crate::service;
#[cfg(not(feature = "mock"))]

/// 创建社区(tested)
pub async fn create_community(
    father_id: Option<u32>,
    bio: String,
    name: String,
    announcement: Option<String>,
    avatar: String,
    pinned: bool,
    status: u8,
    passwd: Option<String>,
) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(3434);
    #[cfg(not(feature = "mock"))]
    {
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        let community = payload::resources::community::Community::new(
            father_id,
            user.user_id,
            &name,
            bio,
            passwd,
            announcement,
            &avatar,
            pinned,
            status,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        let community_id = worker.gen_id()?;
        let community_member_id = worker.gen_id()?;
        let community_admin_id = worker.gen_id()?;

        let community_member = payload::resources::community::member::CommunityMember::new(
            community_member_id,
            1,
            name,
            avatar,
            0,
        );
        let community_admin = payload::resources::community::admin::CommunityAdmin::new(
            1,
            community_id,
            user.user_id,
        );
        service::community::_community::CreateCommunityReq::new(
            community,
            community_id,
            community_member,
            user.user_id,
            community_admin,
            community_admin_id,
        )
        .exec()
        .await?;

        Ok(community_id)
    }
}

/// 更新社区(tested)
pub async fn update_community(
    community_id: u32,
    name: String,
    bio: String,
    passwd: Option<String>,
    announcement: Option<String>,
    avatar: String,
    pinned: bool,
    status: u8,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(());
    #[cfg(not(feature = "mock"))]
    {
        let community = payload::resources::community::info::CommunityInfo::new(
            name,
            bio,
            passwd,
            announcement,
            avatar,
            pinned,
            status,
        );

        crate::service::community::_community::UpdateCommunityReq::new(community, community_id)
            .exec()
            .await?;

        Ok(())
    }
}

/// 删除社区(tested)
pub async fn del_community(community_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(());
    #[cfg(not(feature = "mock"))]
    {
        crate::service::community::_community::DeleteCommunityReq::new(community_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 社区列表(tested)
pub async fn community_list(
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::community::_community::CommunityDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::community::_community::CommunityDetailRes {
                father_id: None,
                user_id: 6546,
                name: "test".to_string(),
                status: 1,
                ..Default::default()
            },
            crate::logic::community::_community::CommunityDetailRes {
                father_id: Some(123),
                user_id: 5435,
                name: "test2".to_string(),
                passwd: Some("asdasd".to_string()),
                pinned: true,
                status: 2,
                ..Default::default()
            },
        ];
        return Ok(list);
    }
    #[cfg(not(feature = "mock"))]
    {
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        Ok(
            crate::service::community::_community::CommunityListReq::new(
                user.user_id,
                page_size,
                offset,
            )
            .exec()
            .await?,
        )
    }
}

/// 社区详情(tested)
pub async fn community_detail(
    community_id: u32,
) -> Result<crate::logic::community::_community::CommunityDetailRes, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let comm = crate::logic::community::_community::CommunityDetailRes {
            father_id: Some(123),
            user_id: 5435,
            name: "test2".to_string(),
            passwd: Some("asdasd".to_string()),
            pinned: true,
            status: 2,
            ..Default::default()
        };
        return Ok(comm);
    }
    #[cfg(not(feature = "mock"))]
    {
        Ok(
            crate::service::community::_community::CommunityDetailReq::new(community_id)
                .exec()
                .await?,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{handler::account::account_list, operator::sqlite::UserState};

    use super::*;

    async fn init() {
        use crate::operator::sqlite::init::DbConnection;
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let res = DbConnection::init_user_database(pri_url.to_string()).await;
        println!("init_user_database res: {res:?}");
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

        let user_id = account_list(1, 0).await.unwrap().pop().unwrap();
        let _ = UserState::init_user_state(user_id.user_id).await;
    }

    #[tokio::test]
    async fn test_create_community() {
        init().await;

        crate::init_log();
        // Test case for creating a community
        let father_id = Some(2);
        let bio = "Community Bio".to_string();
        let name = "Test Community".to_string();
        let announcement = Some("Important Announcement".to_string());
        let avatar = "community_avatar.jpg".to_string();
        let pinned = true;
        let status = 1;
        let passwd = Some("password".to_string());

        let result = create_community(
            father_id,
            bio,
            name,
            announcement,
            avatar,
            pinned,
            status,
            passwd,
        )
        .await;

        assert!(result.is_ok());
        let community_id = result.unwrap();
        assert_ne!(community_id, 0);
    }

    #[tokio::test]
    async fn test_update_community() {
        init().await;
        // Test case for updating a community
        let community_id = 662835200;
        let name = "Updated Community Name".to_string();
        let bio = "Updated Community Bio".to_string();
        let passwd = Some("new_password".to_string());
        let announcement = Some("Updated Announcement".to_string());
        let avatar = "new_community_avatar.jpg".to_string();
        let pinned = false;
        let status = 3;

        let result = update_community(
            community_id,
            name,
            bio,
            passwd,
            announcement,
            avatar,
            pinned,
            status,
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_community_list() {
        init().await;
        // Test case for getting a list of communities
        let _user_id = 121769984;
        let page_size = 10;
        let offset = 0;

        let result = community_list(page_size, offset).await;

        println!("{:#?}", result);
        let community_list = result.unwrap();
        assert!(!community_list.is_empty());
    }

    #[tokio::test]
    async fn test_community_detail() {
        init().await;
        // Test case for getting details of a community
        let community_id = 1442975744;

        let result = community_detail(community_id).await;

        println!("{:#?}", result);
        let _community_detail = result.unwrap();
    }
}
