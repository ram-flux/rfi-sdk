#[cfg(not(feature = "mock"))]
use resource::Action as _;

/// 更新账户
pub async fn update_info(
    user_id: u32,
    account: String,
    gender: u8,
    public_key: String,
    name: String,
    avatar: String,
    bio: String,
) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let account = payload::resources::account::Account::new(
            user_id, public_key, account, gender, name, avatar, bio,
        );
        let account_action = resource::GeneralAction::Update {
            id: user_id,
            resource: account,
        };

        let account_resource = crate::resources::Resources::Account(resource::Command::new(
            user_id as i64,
            account_action,
            "UpdateAvatar".to_string(),
        ));

        let pool = crate::db::USER_SQLITE_POOL.read().await;
        let pool = pool.get_pool().unwrap();
        let _ = account_resource.execute(pool.as_ref()).await;
        Ok(()).into()
    }
}

/// 更新头像
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
