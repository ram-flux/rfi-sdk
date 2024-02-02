#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum QueryResult<T> {
    Vec(Vec<T>),
    One(T),
}

impl<T> From<Vec<T>> for QueryResult<T> {
    fn from(value: Vec<T>) -> Self {
        QueryResult::Vec(value)
    }
}

impl<T> From<T> for QueryResult<T> {
    fn from(value: T) -> Self {
        QueryResult::One(value)
    }
}

pub(crate) trait Query: serde::Serialize + Sized + std::fmt::Debug {
    type Res = QueryResult<Self>;

    async fn query<F, O>(op: F) -> Result<Self::Res, crate::DatabaseError>
    where
        F: FnOnce(
            std::sync::Arc<super::init::PoolSqlite>,
            std::sync::Arc<super::init::PoolSqlite>,
        ) -> O,
        O: std::future::Future<Output = Result<Self::Res, sqlx::Error>>,
    {
        let user_pool = super::init::USER_SQLITE_POOL.read().await;
        let user_pool = user_pool.get_pool()?;

        let pub_pool = super::init::DbConnection::get_pub_connection()?;
        let pub_pool = pub_pool.get_pool()?;
        let res = op(user_pool.clone(), pub_pool.clone()).await.map_err(|e| {
            println!("error: {e}");
            crate::DatabaseError::QueryFailed
        })?;
        // tracing::info!("query result: {res:?}");
        Ok(res)
    }
}

impl<T: serde::Serialize + Sized + std::fmt::Debug> Query for T {}

pub(super) trait SerdeTool: serde::Serialize + Sized {
    fn serde_to_string(self) -> Result<String, crate::Error> {
        serde_json::to_string(&self).map_err(|e| crate::Error::Parse(e.to_string()))
    }

    fn map<F, B>(self, op: F) -> B
    where
        F: FnOnce(Self) -> B,
    {
        op(self)
    }
}

impl<T: serde::Serialize + Sized> SerdeTool for T {}
