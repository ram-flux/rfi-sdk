/// 帖子列表(done, untested)
pub async fn post_list(
    community_id: u32,
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::community::post::PostDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::community::post::PostDetailRes {
                user_id: 6546,
                name: "test".to_string(),
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            crate::logic::community::post::PostDetailRes {
                user_id: 5435,
                name: "test2".to_string(),
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
            crate::service::community::post::PostListReq::new(community_id, page_size, offset)
                .exec()
                .await?,
        )
    }
}

/// 创建帖子(tested)
pub async fn create_post(
    community_id: u32,
    user_id: u32,
    name: String,
    content: String,
    sort_count: i32,
) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(3234).into();
    #[cfg(not(feature = "mock"))]
    {
        let post = payload::resources::community::post::Post::new(
            community_id,
            user_id,
            name,
            content,
            sort_count,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        let post_id = worker.gen_id()?;
        crate::service::community::post::CreatePostReq::new(post, post_id)
            .exec()
            .await?;
        Ok(post_id)
    }
}

// pub async fn update_post(
//     post_id: u32,
//     community_id: u32,
//     user_id: u32,
//     name: String,
//     content: String,
//     sort_count: i32,
// ) -> Result<(), crate::Error> {
//     #[cfg(feature = "mock")]
//     return Ok(()).into();
//     // #[cfg(not(feature = "mock"))]
//     {
//         let post = payload::resources::community::post::Post::new(
//             community_id,
//             user_id,
//             name,
//             content,
//             sort_count,
//         );
//         let mut worker = crate::operator::WrapWorker::worker()?;
//         crate::service::community::post::UpdatePostReq::new(post, post_id)
//             .exec()
//             .await?;
//         Ok(())
//     }
// }

/// 编辑帖子(tested)
pub async fn edit_post(
    post_id: u32,
    name: String,
    content: String,
    _sort_count: i32,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let post_info = payload::resources::community::post::info::PostInfo::new(name, content);
        crate::service::community::post::EditPostReq::new(post_info, post_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 删除帖子(tested)
pub async fn del_post(post_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::community::post::DeletePostReq::new(post_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 帖子详情(tested)
pub async fn post_detail(
    post_id: u32,
) -> Result<crate::logic::community::post::PostDetailRes, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let post = crate::logic::community::post::PostDetailRes {
            user_id: 6565656,
            ..Default::default()
        };
        return Ok(post);
    }
    #[cfg(not(feature = "mock"))]
    {
        Ok(crate::service::community::post::PostDetailReq::new(post_id)
            .exec()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{account::account_list, community::_community::community_list};

    use super::*;

    async fn init_db() {
        use crate::operator::sqlite::init::DbConnection;
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let res = DbConnection::init_user_database(pri_url.to_string()).await;
        println!("init_user_database res: {res:?}");
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;
    }

    // Other test functions from the previous response...

    #[tokio::test]
    async fn test_post_list() {
        init_db().await;
        // Test case for post_list function
        let community_id = 784470016;
        let page_size = 10;
        let offset = 0;

        let result = post_list(community_id, page_size, offset).await;

        println!("{:#?}", result);
        let _post_list = result.unwrap();
        // Include assertions for specific details if needed
    }

    #[tokio::test]
    async fn test_create_post() {
        init_db().await;
        // Test case for create_post function
        let user = account_list(1, 0).await.unwrap().pop().unwrap();
        let community = community_list(1, 0).await.unwrap().pop().unwrap();
        let community_id = community.id;
        let user_id = user.user_id;
        let name = "Test Post".to_string();
        let content = "Post content here".to_string();
        let sort_count = 42;

        let result = create_post(community_id, user_id, name, content, sort_count).await;

        assert!(result.is_ok());
        let post_id = result.unwrap();
        assert_ne!(post_id, 0);
    }

    // #[tokio::test]
    // async fn test_update_post() {
    //     init_db().await;
    //     // Test case for update_post function
    //     let post_id = 12345;
    //     let community_id = 6789;
    //     let user_id = 5432;
    //     let name = "Updated Post Name".to_string();
    //     let content = "Updated post content".to_string();
    //     let sort_count = 55;

    //     let result: Result<(), crate::Error> =
    //         update_post(post_id, community_id, user_id, name, content, sort_count).await;

    //     assert!(result.is_ok());
    // }

    #[tokio::test]
    async fn test_edit_post() {
        init_db().await;
        // Test case for update_post function
        let post_id = 2088898560;
        let name = "Updated Post Name".to_string();
        let content = "Updated post content".to_string();
        let sort_count = 55;

        let result: Result<(), crate::Error> = edit_post(post_id, name, content, sort_count).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_del_post() {
        init_db().await;
        // Test case for del_post function
        let post_id = 2088898560;

        let result: Result<(), crate::Error> = del_post(post_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_detail() {
        init_db().await;
        // Test case for post_detail function
        let post_id = 2088898560;

        let result: Result<crate::logic::community::post::PostDetailRes, crate::Error> =
            post_detail(post_id).await;

        println!("{:#?}", result);
        let _post_detail = result.unwrap();
        // Include assertions for specific details if needed
    }
}
