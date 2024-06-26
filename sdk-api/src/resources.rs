//
//  Copyright 2024 Ram Flux, LLC.
//

use payload::resources::{
    account::{avatar::AccountAvatar, community::AccountCommunity, elf::AccountElf, Account},
    apply::{reply::ApplyReply, Apply},
    chat::{status::ChatStatus, Chat},
    community::{
        admin::{typ::CommunityAdminType, CommunityAdmin},
        info::CommunityInfo,
        member::{typ::CommunityMemberType, CommunityMember},
        post::{info::PostInfo, Post},
        post_reply::{info::PostReplyInfo, PostReply},
        Community,
    },
    contact::{remark::ContactRemark, Contact},
    device::{token::Token, Device},
    elf::Elf,
    favorite::Favorite,
    message::{block::Block, status::MessageStatus, Message},
    nav::Nav,
    settings::{language::Language, Settings},
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Resources {
    // Account
    Account(resource::Command<resource::GeneralAction<Account>>),
    AccountElf(resource::Command<resource::GeneralAction<AccountElf>>),
    AccountCommunity(resource::Command<resource::GeneralAction<AccountCommunity>>),
    AccountAvatar(resource::Command<resource::GeneralAction<AccountAvatar>>),
    // Community
    Community(resource::Command<resource::GeneralAction<Community>>),
    CommunityInfo(resource::Command<resource::GeneralAction<CommunityInfo>>),
    CommunityAdmin(resource::Command<resource::GeneralAction<CommunityAdmin>>),
    CommunityAdminType(resource::Command<resource::GeneralAction<CommunityAdminType>>),
    CommunityMember(resource::Command<resource::GeneralAction<CommunityMember>>),
    CommunityMemberType(resource::Command<resource::GeneralAction<CommunityMemberType>>),
    // Post
    Post(resource::Command<resource::GeneralAction<Post>>),
    PostInfo(resource::Command<resource::GeneralAction<PostInfo>>),
    PostReply(resource::Command<resource::GeneralAction<PostReply>>),
    PostReplyInfo(resource::Command<resource::GeneralAction<PostReplyInfo>>),

    // Device
    Device(resource::Command<resource::GeneralAction<Device>>),
    Token(resource::Command<resource::GeneralAction<Token>>),
    // Message
    Message(resource::Command<resource::GeneralAction<Message>>),
    Block(resource::Command<resource::GeneralAction<Block>>),
    Status(resource::Command<resource::GeneralAction<MessageStatus>>),
    // Apply
    Apply(resource::Command<resource::GeneralAction<Apply>>),
    ApplyReply(resource::Command<resource::GeneralAction<ApplyReply>>),
    // Contact
    Contact(resource::Command<resource::GeneralAction<Contact>>),
    ContactRemark(resource::Command<resource::GeneralAction<ContactRemark>>),
    Elf(resource::Command<resource::GeneralAction<Elf>>),
    // Chat
    Chat(resource::Command<resource::GeneralAction<Chat>>),
    ChatStatus(resource::Command<resource::GeneralAction<ChatStatus>>),
    Favorite(resource::Command<resource::GeneralAction<Favorite>>),
    Nav(resource::Command<resource::GeneralAction<Nav>>),
    // Settings
    Settings(resource::Command<resource::GeneralAction<Settings>>),
    Language(resource::Command<resource::GeneralAction<Language>>),
}

impl resource::Action for Resources {
    async fn execute<'c, E>(&self, executor: E) -> Result<(), resource::Error>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Sqlite>,
    {
        match self {
            // Account
            Resources::Account(r) => r.execute(executor).await,
            Resources::AccountElf(r) => r.execute(executor).await,
            Resources::AccountCommunity(r) => r.execute(executor).await,
            Resources::AccountAvatar(r) => r.execute(executor).await,
            // Community
            Resources::Community(r) => r.execute(executor).await,
            Resources::CommunityInfo(r) => r.execute(executor).await,
            Resources::CommunityAdmin(r) => r.execute(executor).await,
            Resources::CommunityAdminType(r) => r.execute(executor).await,
            Resources::CommunityMember(r) => r.execute(executor).await,
            Resources::CommunityMemberType(r) => r.execute(executor).await,
            // Post
            Resources::Post(r) => r.execute(executor).await,
            Resources::PostInfo(r) => r.execute(executor).await,
            Resources::PostReply(r) => r.execute(executor).await,
            Resources::PostReplyInfo(r) => r.execute(executor).await,
            // Device
            Resources::Device(r) => r.execute(executor).await,
            Resources::Token(r) => r.execute(executor).await,
            // Message
            Resources::Message(r) => r.execute(executor).await,
            Resources::Block(r) => r.execute(executor).await,
            Resources::Status(r) => r.execute(executor).await,
            // Apply
            Resources::Apply(r) => r.execute(executor).await,
            Resources::ApplyReply(r) => r.execute(executor).await,
            // Contact
            Resources::Contact(r) => r.execute(executor).await,
            Resources::ContactRemark(r) => r.execute(executor).await,
            Resources::Elf(r) => r.execute(executor).await,
            // Chat
            Resources::Chat(r) => r.execute(executor).await,
            Resources::ChatStatus(r) => r.execute(executor).await,
            Resources::Favorite(r) => r.execute(executor).await,
            Resources::Nav(r) => r.execute(executor).await,
            // Settings
            Resources::Settings(r) => r.execute(executor).await,
            Resources::Language(r) => r.execute(executor).await,
        }
    }
}

impl resource::Resources for Resources {}
