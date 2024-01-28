#[cfg(not(feature = "mock"))]
use resource::Action as _;

/// 更新账户(tested)
pub async fn update_info(
    account_id: u32,
    user_id: u32,
    account: String,
    gender: u8,
    public_key: String,
    name: String,
    salt: String,
    avatar: String,
    bio: String,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let account = payload::resources::account::Account::new(
            user_id, public_key, account, salt, gender, name, avatar, bio,
        );
        crate::service::account::update_account(account, account_id).await;
        Ok(()).into()
    }
}

/// 更新头像(tested)
pub async fn update_avatar(user_id: u32, avatar: String) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let avatar = payload::resources::account::avatar::Avatar::new(avatar);
        let avatar_action = resource::GeneralAction::Update {
            id: user_id,
            resource: avatar,
        };

        let avatar_resource = crate::resources::Resources::Avatar(resource::Command::new(
            user_id as i64,
            avatar_action,
            "UpdateAvatar".to_string(),
        ));
        let pool = crate::db::USER_SQLITE_POOL.read().await;
        let pool = pool.get_pool().unwrap();
        let _ = avatar_resource.execute(pool.as_ref()).await;
        Ok(()).into()
    }
}

/// 账号详情(tested)
pub async fn account_detail(
    account_id: u32,
) -> crate::response::Response<
    crate::sqlite_operator::QueryResult<payload::resources::account::Account>,
> {
    #[cfg(feature = "mock")]
    {
        let comm = payload::resources::account::Account {
            user_id: 5435,
            name: "test2".to_string(),
            // created_at: payload::utils::time::now(),
            // updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(crate::sqlite_operator::QueryResult::One(comm)).into();
    }
    #[cfg(not(feature = "mock"))]
    {
        use crate::sqlite_operator::Query as _;
        payload::resources::account::Account::query(async move |user_pool, pub_pool| {
            let sql = "SELECT user_id, public_key, account, salt, gender, name, avatar,
                bio, created_at, updated_at 
                FROM account 
                WHERE id =$1;";
            sqlx::query_as::<sqlx::Sqlite, payload::resources::account::Account>(sql)
                .bind(account_id)
                .fetch_one(user_pool.as_ref())
                .await
                .map(Into::into)
        })
        .await
        .map_err(|e| crate::Error::BadRequest(crate::AccountError::DatabaseError(e).into()))
        .into()
    }
}

/// 添加社区
pub async fn add_community(
    community: u32,
    user_id: u32,
    content: String,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

/// 退出社区
pub async fn quit_community(user_id: u32, community_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

/// 添加帖子
pub async fn add_contact(
    friend_id: u32,
    user_id: u32,
    content: String,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

#[cfg(test)]
mod test {
    use crate::db::DbConnection;

    #[tokio::test]
    async fn test_update_account() {
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let _ = DbConnection::init_user_database(pri_url.to_string()).await;
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

        let account_id = 1308758018;
        let user_id = 123;
        let account = "234234".to_string();
        let gender = 1;
        let public_key = "gffd".to_string();
        let name = "adsdad".to_string();
        let salt = "salt".to_string();
        let avatar = "avatar".to_string();
        let bio = "bio".to_string();
        let res = crate::command::account::update_info(
            account_id, user_id, account, gender, public_key, name, salt, avatar, bio,
        )
        .await;
        println!("res: {res:?}");
    }

    #[tokio::test]
    async fn test_update_avatar() {
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let _ = DbConnection::init_user_database(pri_url.to_string()).await;
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

        let user_id = 123;
        let avatar = "dfdsfdsf".to_string();
        let res = crate::command::account::update_avatar(user_id, avatar).await;
        println!("res: {res:?}");
    }

    #[tokio::test]
    async fn test_account_detail() {
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let _ = DbConnection::init_user_database(pri_url.to_string()).await;
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

        let account_id = 1308758018;
        let res = crate::command::account::account_detail(account_id).await;
        println!("res: {res:#?}");
    }

    #[tokio::test]
    async fn test_sqlite_query_one() {
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let _ = DbConnection::init_user_database(pri_url.to_string()).await;
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

        let community_id = 1614942208;
        let res = crate::command::community::info::community_detail(community_id).await;
        println!("res: {res:#?}");
    }
}
