use crate::{operator::sqlite::query::Query as _, service};
#[cfg(not(feature = "mock"))]
use resource::Action as _;

/// 创建社区(done, untested)
pub async fn create_community(
    user_id: u32,
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
        let community = payload::resources::community::Community::new(
            father_id,
            user_id,
            name,
            bio,
            passwd,
            announcement,
            avatar,
            pinned,
            status,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        let community_id = worker.gen_id()?;
        service::community::_community::CreateCommunityReq::new(community, community_id)
            .exec()
            .await?;

        Ok(community_id)
    }
}

/// 更新社区(done, untested)
pub async fn update_community(
    community_id: u32,
    name: String,
    bio: String,
    passwd: Option<String>,
    announcement: Option<String>,
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
            pinned,
            status,
        );

        crate::service::community::_community::UpdateCommunityReq::new(community, community_id)
            .exec()
            .await?;

        Ok(())
    }
}

/// 删除社区(done, untested)
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

/// 社区列表(done, untested)
pub async fn community_list(
    user_id: u32,
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
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::community::_community::CommunityListReq::new(
                user_id, page_size, offset,
            )
            .exec()
            .await?,
        )
    }
}

/// 社区详情(done, untested)
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
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::community::_community::CommunityDetailReq::new(community_id)
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
    async fn test_create_community() {
        init_db().await;
        // Test case for creating a community
        let user_id = 121769984;
        let father_id = Some(2);
        let bio = "Community Bio".to_string();
        let name = "Test Community".to_string();
        let announcement = Some("Important Announcement".to_string());
        let avatar = "community_avatar.jpg".to_string();
        let pinned = true;
        let status = 1;
        let passwd = Some("password".to_string());

        let result = create_community(
            user_id,
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

        // Add assertions or checks based on the expected behavior
        // For example:
        assert!(result.is_ok());
        let community_id = result.unwrap();
        assert_ne!(community_id, 0);
    }

    #[tokio::test]
    async fn test_update_community() {
        init_db().await;
        // Test case for updating a community
        let community_id = 123;
        let name = "Updated Community Name".to_string();
        let bio = "Updated Community Bio".to_string();
        let passwd = Some("new_password".to_string());
        let announcement = Some("Updated Announcement".to_string());
        let pinned = false;
        let status = 2;

        let result = update_community(
            community_id,
            name,
            bio,
            passwd,
            announcement,
            pinned,
            status,
        )
        .await;

        // Add assertions or checks based on the expected behavior
        // For example:
        assert!(result.is_ok());
    }

    // Similarly, you can add tests for other functions like del_community, community_list, community_detail
    // ...

    #[tokio::test]
    async fn test_community_list() {
        init_db().await;
        // Test case for getting a list of communities
        let user_id = 1;
        let page_size = 10;
        let offset = 0;

        let result = community_list(user_id, page_size, offset).await;

        // Add assertions or checks based on the expected behavior
        // For example:
        assert!(result.is_ok());
        let community_list = result.unwrap();
        assert!(!community_list.is_empty());
    }

    #[tokio::test]
    async fn test_community_detail() {
        init_db().await;
        // Test case for getting details of a community
        let community_id = 456;

        let result = community_detail(community_id).await;

        // Add assertions or checks based on the expected behavior
        // For example:
        assert!(result.is_ok());
        let community_detail = result.unwrap();
        assert_eq!(community_detail.user_id, 5435);
    }
}
