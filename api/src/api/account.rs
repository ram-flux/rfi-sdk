use crate::operator::sqlite::query::QueryResult;

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
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let account = payload::resources::account::Account::new(
            user_id, public_key, account, salt, gender, name, avatar, bio,
        );
        crate::logic::update::update_account(account, account_id).await;
        Ok(())
    }
}

/// 更新头像(tested)
pub async fn update_avatar(account_id: u32, avatar: String) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let account = account_detail(account_id).await?;
        if let QueryResult::One(data) = account {
            let avatar = payload::resources::account::avatar::Avatar::new(avatar);

            crate::service::account::UpdateAvatarReq::new(avatar, account_id)
                .exec()
                .await?;
        }

        Ok(())
    }
}

/// 账号详情(tested)
pub async fn account_detail(
    user_id: u32,
) -> Result<QueryResult<crate::logic::account::AccountDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let comm = crate::logic::account::AccountDetailRes {
            user_id: 5435,
            name: "test2".to_string(),
            ..Default::default()
        };
        return Ok(QueryResult::One(comm)).into();
    }
    #[cfg(not(feature = "mock"))]
    {
        Ok(crate::service::account::AccountDetailReq::new(user_id)
            .exec()
            .await?)
    }
}

/// 添加社区
pub async fn add_community(
    community: u32,
    user_id: u32,
    content: String,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

/// 退出社区
pub async fn quit_community(user_id: u32, community_id: u32) -> Result<(), crate::Error> {
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
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

#[cfg(test)]
mod test {
    use crate::operator::sqlite::init::DbConnection;

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
        let res = crate::api::account::update_info(
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
        let res = crate::api::account::update_avatar(user_id, avatar).await;
        println!("res: {res:?}");
    }

    #[tokio::test]
    async fn test_account_detail() {
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let _ = DbConnection::init_user_database(pri_url.to_string()).await;
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

        let account_id = 1308758018;
        let res = crate::api::account::account_detail(account_id).await;
        println!("res: {res:#?}");
    }

    // #[tokio::test]
    // async fn test_sqlite_query_one() {
    //     let pri_url = "sqlite://test_pri.db";
    //     let pub_url = "sqlite://test_pub.db";
    //     let _ = DbConnection::init_user_database(pri_url.to_string()).await;
    //     let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

    //     let community_id = 1614942208;
    //     let res = crate::api::community::info::community_detail(community_id).await;
    //     println!("res: {res:#?}");
    // }
}
