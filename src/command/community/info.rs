#[cfg(not(feature = "mock"))]
use crate::sqlite_operator::Query as _;
#[cfg(not(feature = "mock"))]
use resource::Action as _;

/// 社区列表
pub async fn community_list(
    user_id: u32,
) -> crate::response::Response<
    crate::sqlite_operator::QueryResult<payload::resources::community::Community>,
> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            payload::resources::community::Community {
                father_id: None,
                user_id: 6546,
                name: "test".to_string(),
                status: 1,
                ..Default::default()
            },
            payload::resources::community::Community {
                father_id: Some(123),
                user_id: 5435,
                name: "test2".to_string(),
                passwd: Some("asdasd".to_string()),
                pinned: true,
                status: 2,
                ..Default::default()
            },
        ];
        return Ok(crate::sqlite_operator::QueryResult::Vec(list)).into();
    }
    #[cfg(not(feature = "mock"))]
    payload::resources::community::Community::query(
        async move |user_pool, pub_pool| {
        let sql =
            "SELECT id, father_id, user_id, name, bio, passwd, announcement, pinned, status, created_at, 
            updated_at FROM community 
            WHERE user_id =$1;";
        sqlx::query_as::<sqlx::Sqlite, _>(sql)
            .bind(user_id)
            .fetch_all(user_pool.as_ref())
            .await
            .map(Into::into)
    })
    .await
    .map_err(|e| crate::Error::BadRequest(
        crate::CommunityError::DatabaseError(e).into()
    ))
    .into()
}

/// 社区详情
pub async fn community_detail(
    user_id: u32,
) -> crate::response::Response<
    crate::sqlite_operator::QueryResult<payload::resources::community::Community>,
> {
    #[cfg(feature = "mock")]
    {
        let comm = payload::resources::community::Community {
            father_id: Some(123),
            user_id: 5435,
            name: "test2".to_string(),
            passwd: Some("asdasd".to_string()),
            pinned: true,
            status: 2,
            // created_at: payload::utils::time::now(),
            // updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(crate::sqlite_operator::QueryResult::One(comm)).into();
    }
    #[cfg(not(feature = "mock"))]
    {
        payload::resources::community::Community::query(async move |user_pool, pub_pool| {
            let sql =
                "SELECT id, father_id, user_id, name, bio, passwd, announcement, pinned, status FROM community 
                WHERE user_id =$1;";
            sqlx::query_as::<sqlx::Sqlite, payload::resources::community::Community>(sql)
                .bind(user_id)
                .fetch_one(user_pool.as_ref())
                .await
                .map(Into::into)
        })
        .await
        .map_err(|e| crate::Error::BadRequest(
            crate::InitDatabaseError::DatabaseError(e).into()
        )).into()
    }
}

/// 创建社区
pub async fn create_community(
    user_id: u32,
    father_id: Option<u32>,
    bio: String,
    name: String,
    announcement: Option<String>,
    pinned: bool,
    status: u8,
    passwd: Option<String>,
) -> crate::response::Response<u32> {
    #[cfg(feature = "mock")]
    return Ok(3434).into();
    #[cfg(not(feature = "mock"))]
    {
        let community = payload::resources::community::Community::new(
            father_id,
            user_id,
            name,
            bio,
            passwd,
            announcement,
            pinned,
            status,
        );
        let id = payload::utils::gen_id();
        let community_action = resource::GeneralAction::Upsert {
            id: Some(id),
            resource: community,
        };

        let community_resource = crate::resources::Resources::Community(resource::Command::new(
            user_id as i64,
            community_action,
            "UpsertCommunity".to_string(),
        ));

        let pool = crate::db::USER_SQLITE_POOL.read().await;
        let pool = pool.get_pool().unwrap();
        let _ = community_resource.execute(pool.as_ref()).await;
        Ok(id).into()
    }
}

/// 更新社区
pub async fn update_community(
    community_id: u32,
    name: String,
    bio: String,
    passwd: Option<String>,
    announcement: Option<String>,
    pinned: bool,
    status: u8,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

/// 删除社区
pub async fn del_community(community_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
