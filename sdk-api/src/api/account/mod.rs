//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod param;

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
) -> crate::response::Response<()> {
    crate::handler::account::update_info(
        account_id, account, gender, public_key, name, salt, avatar, bio,
    )
    .await
    .into()
}

/// 更新头像(tested)
pub async fn update_avatar(account_id: u32, avatar: String) -> crate::response::Response<()> {
    crate::handler::account::update_avatar(account_id, avatar)
        .await
        .into()
}

/// 账号详情(tested)
pub async fn account_detail(
    user_id: u32,
) -> crate::response::Response<crate::logic::account::AccountDetailRes> {
    crate::handler::account::account_detail(user_id)
        .await
        .into()
}

/// 账号列表(tested)
pub async fn account_list(
    page_size: u16,
    offset: u16,
) -> crate::response::Response<Vec<crate::logic::account::AccountDetailRes>> {
    crate::handler::account::account_list(page_size, offset)
        .await
        .into()
}

/// 添加社区(done, untested)
pub async fn add_community(community_id: u32) -> crate::response::Response<()> {
    crate::handler::account::add_community(community_id)
        .await
        .into()
}

/// 创建账户机器人(done, untested)
pub async fn create_account_elf(
    name: String,
    avatar: String,
) -> crate::response::Response<param::CreateAccountElfRes> {
    param::CreateAccountElfRes::create_account_elf(name, avatar)
        .await
        .into()
}

/// 退出社区(done, untested)
pub async fn quit_community(community_id: u32) -> crate::response::Response<()> {
    crate::handler::account::quit_community(community_id)
        .await
        .into()
}
