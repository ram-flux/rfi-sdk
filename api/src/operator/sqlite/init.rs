use sqlx::{Pool, Sqlite};

pub static USER_SQLITE_POOL: once_cell::sync::Lazy<
    tokio::sync::RwLock<std::sync::Arc<DbConnection>>,
> = once_cell::sync::Lazy::new(|| {
    tokio::sync::RwLock::new(std::sync::Arc::new(DbConnection::default()))
});
// pub static USER_SQLITE_POOL: once_cell::sync::Lazy<
//     std::sync::Arc<tokio::sync::RwLock<Option<DbConnection>>>,
// > = once_cell::sync::Lazy::new(|| std::sync::Arc::new(tokio::sync::RwLock::new(None)));

pub static PUBLIC_SQLITE_POOL: once_cell::sync::Lazy<
    once_cell::sync::OnceCell<std::sync::Arc<DbConnection>>,
> = once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);

#[derive(Debug, Default, Clone)]
pub struct DbConnection {
    pub conn: Option<std::sync::Arc<PoolSqlite>>,
    pub uri: String,
    pub migrator: Migrator,
}

#[derive(Debug, Default, Clone)]
pub enum Migrator {
    #[default]
    Pub,
    Pri,
}

pub type PoolSqlite = Pool<Sqlite>;

impl DbConnection {
    pub async fn init_user_database(url: String) -> Result<(), crate::DatabaseError> {
        let mut conn = DbConnection::default();
        conn.set_migrator(Migrator::Pri);
        let migrator = conn.migrator();
        let db = conn.set_uri(Some(url)).init_database().await?;

        let mut pool = USER_SQLITE_POOL.write().await;
        *pool = std::sync::Arc::new(db);

        Ok(())
    }

    pub async fn init_pub_database(url: String) -> Result<(), crate::DatabaseError> {
        let mut conn = DbConnection::default();
        conn.set_migrator(Migrator::Pub);
        let migrator = conn.migrator();
        let db = conn.set_uri(Some(url)).init_database().await?;

        let _ = PUBLIC_SQLITE_POOL.set(std::sync::Arc::new(db));

        Ok(())
    }

    pub fn get_pub_connection<'a>() -> Result<&'a std::sync::Arc<DbConnection>, crate::DatabaseError>
    {
        PUBLIC_SQLITE_POOL
            .get()
            .ok_or(crate::DatabaseError::GetPublicSqliteConnFailed)
    }

    pub fn migrator(&self) -> sqlx::migrate::Migrator {
        match self.migrator {
            Migrator::Pub => sqlx::migrate!("./schema/public/migrations"),
            Migrator::Pri => sqlx::migrate!("./schema/private/migrations"),
        }
    }

    pub(crate) fn set_migrator(&mut self, migrator: Migrator) {
        self.migrator = migrator
    }

    pub async fn run_migrator(&self) -> Result<(), crate::DatabaseError> {
        let sqlite_pool = sqlx::Pool::<sqlx::Sqlite>::connect(&self.uri).await?;
        // let migrator = self.migrator();
        // self.migrator
        //     .as_ref()
        //     .ok_or(crate::Error::BadRequest(
        //         crate::InitDatabaseError::DatabaseError(crate::DatabaseError::MigratorGetFailed)
        //             .into(),
        //     ))?
        self.migrator()
            .run(&sqlite_pool)
            .await
            .map_err(|_| crate::DatabaseError::MigrationRunFailed)
    }

    pub fn set_uri(mut self, sqlite_url: Option<String>) -> Self {
        let sqlite_url = sqlite_url.map_or("sqlite://rf.db".to_owned(), |db| db);
        self.uri = sqlite_url;
        self
    }

    pub async fn init_database(mut self) -> Result<DbConnection, crate::DatabaseError> {
        // tracing::info!("sqlite_url: {:?}", self.uri);
        let sqlite_url = &self.uri;
        // tracing::info!("[init_database] sqlite_url: {sqlite_url}");
        async fn _create_database(sqlite_url: &str) -> Result<(), crate::DatabaseError> {
            sqlx::Sqlite::create_database(sqlite_url)
                .await
                .map_err(|_| crate::DatabaseError::DatabaseCreateFailed)
        }

        use sqlx::migrate::MigrateDatabase as _;
        if !sqlx::Sqlite::database_exists(sqlite_url)
            .await
            .unwrap_or(false)
        {
            _create_database(sqlite_url).await?;
        };
        let sqlite_pool = sqlx::Pool::<sqlx::Sqlite>::connect(sqlite_url)
            .await
            .map_err(|_| crate::DatabaseError::DatabaseConnectFailed)?;
        let sqlite_pool = if self
            .migrator()
            // .as_ref()
            // .ok_or(crate::DatabaseError::MigratorGetFailed)?
            .run(&sqlite_pool)
            .await
            .is_err()
        {
            tracing::error!("migrate filed: remove files & create database again");
            sqlite_pool.close().await;
            // let storage = STORAGE.get().unwrap();
            sqlx::Sqlite::drop_database(sqlite_url)
                .await
                .map_err(|_| crate::DatabaseError::DatabaseDropFailed)?;
            _create_database(sqlite_url).await?;

            let sqlite_pool = sqlx::Pool::<sqlx::Sqlite>::connect(sqlite_url)
                .await
                .map_err(|_| crate::DatabaseError::DatabaseConnectFailed)?;
            self.migrator()
                // .as_ref()
                // .ok_or(crate::DatabaseError::MigratorGetFailed)?
                .run(&sqlite_pool)
                .await
                .map_err(|e| {
                    println!("[init_database] migrate failed: {e}");
                    crate::DatabaseError::MigrationRunFailed
                })?;
            sqlite_pool
        } else {
            sqlite_pool
        };
        self.conn = Some(std::sync::Arc::new(sqlite_pool));
        Ok(self)
    }

    pub fn get_pool(&self) -> Result<&std::sync::Arc<Pool<Sqlite>>, crate::DatabaseError> {
        self.conn.as_ref().ok_or({
            match self.migrator {
                Migrator::Pub => crate::DatabaseError::GetPublicSqlitePoolFailed,
                Migrator::Pri => crate::DatabaseError::GetUserSqlitePoolFailed,
            }
        })
    }

    pub async fn get_uri(&self) -> String {
        self.uri.clone()
    }
}
