use payload::resources::{
    account::{avatar::Avatar, community::AccountCommunity, Account},
    community::Community,
    device::{token::Token, Device},
    message::Message,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Resources {
    Account(resource::Command<resource::GeneralAction<Account>>),
    Avatar(resource::Command<resource::GeneralAction<Avatar>>),
    Community(resource::Command<resource::GeneralAction<Community>>),
    AccountCommunity(resource::Command<resource::GeneralAction<AccountCommunity>>),
    Device(resource::Command<resource::GeneralAction<Device>>),
    Token(resource::Command<resource::GeneralAction<Token>>),
    Message(resource::Command<resource::GeneralAction<Message>>),
}

impl resource::Action for Resources {
    async fn execute<'c, E>(&self, executor: E) -> Result<(), resource::Error>
    where
        E: sqlx::prelude::Executor<'c, Database = sqlx::Sqlite>,
    {
        match self {
            Resources::Avatar(r) => r.execute(executor).await,
            Resources::Account(r) => r.execute(executor).await,
            Resources::Community(r) => r.execute(executor).await,
            Resources::AccountCommunity(r) => r.execute(executor).await,
            Resources::Device(r) => r.execute(executor).await,
            Resources::Token(r) => r.execute(executor).await,
            Resources::Message(r) => r.execute(executor).await,
        }
    }
}

impl resource::Resources for Resources {}
