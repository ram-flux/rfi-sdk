use payload::resources::{
    account::{avatar::Avatar, Account},
    community::Community,
    device::{token::Token, Device},
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Resources {
    Account(resource::Command<resource::GeneralAction<Account>>),
    Avatar(resource::Command<resource::GeneralAction<Avatar>>),
    Community(resource::Command<resource::GeneralAction<Community>>),
    Device(resource::Command<resource::GeneralAction<Device>>),
    Token(resource::Command<resource::GeneralAction<Token>>),
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
            Resources::Device(r) => r.execute(executor).await,
            Resources::Token(r) => r.execute(executor).await,
        }
    }
}

impl resource::Resources for Resources {}
