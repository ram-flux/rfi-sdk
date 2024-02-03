use payload::resources::{
    account::{avatar::Avatar, community::AccountCommunity, elf::AccountElf, Account},
    apply::{reply::ApplyReply, Apply},
    chat::{status::ChatStatus, Chat},
    community::{
        admin::{typ::CommunityAdminType, CommunityAdmin},
        info::CommunityInfo,
        member::CommunityMember,
        post::Post,
        post_reply::PostReply,
        Community,
    },
    contact::Contact,
    device::{token::Token, Device},
    elf::Elf,
    favorite::Favorite,
    message::{block::Block, status::Status, Message},
    nav::Nav,
    settings::Settings,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Resources {
    // Account
    Account(resource::Command<resource::GeneralAction<Account>>),
    AccountElf(resource::Command<resource::GeneralAction<AccountElf>>),
    AccountCommunity(resource::Command<resource::GeneralAction<AccountCommunity>>),
    Avatar(resource::Command<resource::GeneralAction<Avatar>>),
    // Community
    Community(resource::Command<resource::GeneralAction<Community>>),
    CommunityInfo(resource::Command<resource::GeneralAction<CommunityInfo>>),
    CommunityAdmin(resource::Command<resource::GeneralAction<CommunityAdmin>>),
    CommunityAdminType(resource::Command<resource::GeneralAction<CommunityAdminType>>),
    Member(resource::Command<resource::GeneralAction<CommunityMember>>),
    Post(resource::Command<resource::GeneralAction<Post>>),
    PostReply(resource::Command<resource::GeneralAction<PostReply>>),

    // Device
    Device(resource::Command<resource::GeneralAction<Device>>),
    Token(resource::Command<resource::GeneralAction<Token>>),
    // Message
    Message(resource::Command<resource::GeneralAction<Message>>),
    Block(resource::Command<resource::GeneralAction<Block>>),
    Status(resource::Command<resource::GeneralAction<Status>>),
    // Apply
    Apply(resource::Command<resource::GeneralAction<Apply>>),
    ApplyReply(resource::Command<resource::GeneralAction<ApplyReply>>),
    Contact(resource::Command<resource::GeneralAction<Contact>>),
    Elf(resource::Command<resource::GeneralAction<Elf>>),
    // Chat
    Chat(resource::Command<resource::GeneralAction<Chat>>),
    ChatStatus(resource::Command<resource::GeneralAction<ChatStatus>>),
    Favorite(resource::Command<resource::GeneralAction<Favorite>>),
    Nav(resource::Command<resource::GeneralAction<Nav>>),
    Settings(resource::Command<resource::GeneralAction<Settings>>),
}

impl resource::Action for Resources {
    async fn execute<'c, E>(&self, executor: E) -> Result<(), resource::Error>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Sqlite>,
    {
        match self {
            Resources::Account(r) => r.execute(executor).await,
            Resources::AccountElf(r) => r.execute(executor).await,
            Resources::AccountCommunity(r) => r.execute(executor).await,
            Resources::Avatar(r) => r.execute(executor).await,
            Resources::Community(r) => r.execute(executor).await,
            Resources::CommunityInfo(r) => r.execute(executor).await,
            Resources::CommunityAdmin(r) => r.execute(executor).await,
            Resources::CommunityAdminType(r) => r.execute(executor).await,
            Resources::Member(r) => r.execute(executor).await,
            Resources::Post(r) => r.execute(executor).await,
            Resources::PostReply(r) => r.execute(executor).await,
            Resources::Device(r) => r.execute(executor).await,
            Resources::Token(r) => r.execute(executor).await,
            Resources::Message(r) => r.execute(executor).await,
            Resources::Block(r) => r.execute(executor).await,
            Resources::Status(r) => r.execute(executor).await,
            Resources::Apply(r) => r.execute(executor).await,
            Resources::ApplyReply(r) => r.execute(executor).await,
            Resources::Contact(r) => r.execute(executor).await,
            Resources::Elf(r) => r.execute(executor).await,
            Resources::Chat(r) => r.execute(executor).await,
            Resources::ChatStatus(r) => r.execute(executor).await,
            Resources::Favorite(r) => r.execute(executor).await,
            Resources::Nav(r) => r.execute(executor).await,
            Resources::Settings(r) => r.execute(executor).await,
        }
    }
}

impl resource::Resources for Resources {}
