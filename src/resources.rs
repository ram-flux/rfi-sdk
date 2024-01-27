use payload::resources::{
    account::{avatar::Avatar, Account},
    community::Community,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Resources {
    Account(resource::Command<resource::GeneralAction<Account>>),
    Avatar(resource::Command<resource::GeneralAction<Avatar>>),
    Community(resource::Command<resource::GeneralAction<Community>>),
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
        }
    }
}

impl resource::Resources for Resources {}
