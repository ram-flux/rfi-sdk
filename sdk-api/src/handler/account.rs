//
//  Copyright 2024 Ram Flux, LLC.
//


/// 更新账户(tested)
pub async fn update_info(
    account_id: u32,
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
            public_key, account, salt, gender, name, avatar, bio,
        );
        crate::logic::update::update_account(account, account_id).await?;
        Ok(())
    }
}

/// 更新头像(tested)
pub async fn update_avatar(account_id: u32, avatar: String) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        let account = account_detail(account_id).await?;

        let avatar = payload::resources::account::avatar::AccountAvatar::new(avatar);

        crate::service::account::UpdateAvatarReq::new(avatar, account.user_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 账号详情(tested)
pub async fn account_detail(
    user_id: u32,
) -> Result<crate::logic::account::AccountDetailRes, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let comm = crate::logic::account::AccountDetailRes {
            user_id: 5435,
            name: "test2".to_string(),
            ..Default::default()
        };
        return Ok(comm).into();
    }
    #[cfg(not(feature = "mock"))]
    {
        Ok(crate::service::account::AccountDetailReq::new(user_id)
            .exec()
            .await?)
    }
}

/// 账号列表(tested)
pub async fn account_list(
    page_size: u16,
    offset: u16,
) -> Result<Vec<crate::logic::account::AccountDetailRes>, crate::Error> {
    #[cfg(feature = "mock")]
    {
        let comm = vec![crate::logic::account::AccountDetailRes {
            user_id: 5435,
            name: "test2".to_string(),
            ..Default::default()
        }];
        return Ok(comm).into();
    }
    #[cfg(not(feature = "mock"))]
    {
        Ok(
            crate::service::account::AccountListReq::new(page_size, offset)
                .exec()
                .await?,
        )
    }
}

/// 添加社区(done, untested)
pub async fn add_community(community_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let community = super::community::_community::community_detail(community_id).await?;
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        let community = payload::resources::account::community::AccountCommunity::new(
            community_id,
            user.user_id,
            community.name,
            community.avatar,
        );

        crate::service::account::AddCommunityReq::new(community, community_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 创建账户机器人(done, untested)
pub async fn create_account_elf(name: String, avatar: String) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(111);
    // #[cfg(not(feature = "mock"))]
    {
        let mut worker = crate::operator::WrapWorker::worker()?;
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        let account_elf_id = worker.gen_id()?;
        let account_elf = payload::resources::account::elf::AccountElf::new(
            account_elf_id,
            user.user_id,
            name,
            avatar,
        );

        crate::service::account::elf::AddAccountElfReq::new(account_elf, account_elf_id)
            .exec()
            .await?;
        Ok(account_elf_id)
    }
}

/// 退出社区(done, untested)
pub async fn quit_community(community_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::service::account::QuitCommunityReq::new(community_id)
            .exec()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::operator::sqlite::UserState;

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
    async fn test_update_account() {
        init().await;

        let account_id = 1308758018;
        let account = "234234".to_string();
        let gender = 1;
        let public_key = "gffd".to_string();
        let name = "adsdad".to_string();
        let salt = "salt".to_string();
        let avatar = "avatar".to_string();
        let bio = "bio".to_string();
        let res = crate::api::account::update_info(
            account_id, account, gender, public_key, name, salt, avatar, bio,
        )
        .await;
        println!("res: {res:?}");
    }

    #[tokio::test]
    async fn test_update_avatar() {
        init().await;

        let user_id = 123;
        let avatar = "dfdsfdsf".to_string();
        let res = crate::api::account::update_avatar(user_id, avatar).await;
        println!("res: {res:?}");
    }

    #[tokio::test]
    async fn test_account_detail() {
        init().await;

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
