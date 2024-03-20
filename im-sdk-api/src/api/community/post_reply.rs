/// 回复帖子列表(tested)
pub async fn post_reply_list(
    post_id: u32,
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::community::post_reply::PostReplyDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::community::post_reply::PostReplyDetailRes {
                user_id: 6565656,
                post_id: 2343,
                content: "test".to_string(),
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            crate::logic::community::post_reply::PostReplyDetailRes {
                user_id: 6565656,
                post_id: 2343,
                content: "test".to_string(),
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
            crate::service::community::post_reply::PostReplyListReq::new(
                post_id, page_size, offset,
            )
            .exec()
            .await?,
        )
    }
}

/// 回复帖子(tested)
pub async fn reply_post(
    community_id: u32,
    post_id: u32,
    user_id: u32,
    content: String,
    sort: i32,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let post_reply = payload::resources::community::post_reply::PostReply::new(
            community_id,
            user_id,
            post_id,
            content,
            sort,
        );
        let mut worker = crate::operator::WrapWorker::worker()?;
        let post_reply_id = worker.gen_id()?;
        crate::service::community::post_reply::ReplyPostReq::new(post_reply, post_reply_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 更新帖子回复(done, untested)
// pub async fn update_post_reply(
//     post_reply_id: u32,
//     community_id: u32,
//     user_id: u32,
//     post_id: u32,
//     content: String,
//     sort: i32,
// ) -> Result<(), crate::Error> {
//     #[cfg(feature = "mock")]
//     return Ok(()).into();
//     #[cfg(not(feature = "mock"))]
//     {
//         let post_reply = payload::resources::community::post_reply::PostReply::new(
//             community_id,
//             user_id,
//             post_id,
//             content,
//             sort,
//         );
//         crate::service::community::post_reply::UpdatePostReplyReq::new(post_reply, post_reply_id)
//             .exec()
//             .await?;
//         Ok(())
//     }
// }

/// 编辑帖子回复(tested)
pub async fn edit_post_reply(
    post_reply_id: u32,
    user_id: u32,
    content: String,
    _sort: i32,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let post_reply = post_reply_detail(post_reply_id).await?;
        if post_reply.user_id != user_id {
            return Err(crate::Error::UnAuthorize);
        }
        let post_reply_info =
            payload::resources::community::post_reply::info::PostReplyInfo::new(content);
        crate::service::community::post_reply::EditPostReplyReq::new(
            post_reply_info,
            post_reply_id,
        )
        .exec()
        .await?;
        Ok(())
    }
}

/// 删除帖子回复(tested)
pub async fn del_post_reply(post_reply_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::community::post_reply::DeletePostReplyReq::new(post_reply_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 帖子回复详情(tested)
pub async fn post_reply_detail(
    post_reply_id: u32,
) -> Result<crate::logic::community::post_reply::PostReplyDetailRes, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let post_reply = crate::logic::community::post_reply::PostReplyDetailRes {
            user_id: 6565656,
            post_id: 2343,
            content: "test".to_string(),
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(post_reply);
    }
    // #[cfg(not(feature = "mock"))]
    {
        Ok(
            crate::service::community::post_reply::PostReplyDetailReq::new(post_reply_id)
                .exec()
                .await?,
        )
    }
}
#[cfg(test)]
mod tests {
    use crate::api::{
        account::account_list,
        community::{_community::community_list, post::post_list},
    };

    use super::*;

    async fn init_db() {
        use crate::operator::sqlite::init::DbConnection;
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let res = DbConnection::init_user_database(pri_url.to_string()).await;
        println!("init_user_database res: {res:?}");
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;
    }

    // Previous test functions...

    #[tokio::test]
    async fn test_post_reply_list() {
        init_db().await;
        // Test case for post_reply_list function
        let _user = account_list(1, 0).await.unwrap().pop().unwrap();
        let community = community_list(1, 0).await.unwrap().pop().unwrap();
        let post = post_list(community.id, 1, 0).await.unwrap().pop().unwrap();
        let post_id = post.id;
        let page_size = 10;
        let offset = 0;

        let result = post_reply_list(post_id, page_size, offset).await;

        println!("{:#?}", result);
        let _reply_list = result.unwrap();
        // Include assertions for specific details if needed
    }

    #[tokio::test]
    async fn test_reply_post() {
        init_db().await;
        // Test case for reply_post function
        let user = account_list(1, 0).await.unwrap().pop().unwrap();
        let community = community_list(1, 0).await.unwrap().pop().unwrap();
        let post = post_list(community.id, 1, 0).await.unwrap().pop().unwrap();

        let community_id = community.id;
        let user_id = user.user_id;
        let post_id = post.id;

        let content = "Reply content here".to_string();
        let sort_count = 42;

        let result = reply_post(community_id, post_id, user_id, content, sort_count).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_edit_post_reply() {
        init_db().await;
        // Test case for update_post_reply function
        let post_reply_id = 1891766272;
        let user_id = 3309441024;
        let content = "Updated reply content".to_string();
        let sort_count = 55;

        let result: Result<(), crate::Error> =
            edit_post_reply(post_reply_id, user_id, content, sort_count).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_del_post_reply() {
        init_db().await;
        // Test case for del_post_reply function
        let post_reply_id = 1891766272;

        let result: Result<(), crate::Error> = del_post_reply(post_reply_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_reply_detail() {
        init_db().await;
        // Test case for post_reply_detail function
        let post_reply_id = 1891766272;

        let result: Result<crate::logic::community::post_reply::PostReplyDetailRes, crate::Error> =
            post_reply_detail(post_reply_id).await;

        println!("{:#?}", result);
        let _reply_detail = result.unwrap();
        // Include assertions for specific details if needed
    }
}
