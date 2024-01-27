#![feature(try_trait_v2, async_closure, associated_type_defaults)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(clippy::too_many_arguments)]
pub mod command;
pub mod db;
pub mod error;
pub mod resources;
pub mod response;
pub mod sqlite_operator;

#[cfg(not(feature = "mock"))]
pub(crate) use error::api::community::CommunityError;

pub(crate) use error::{
    api::init_database::InitDatabaseError, bad_request::BadRequest,
    common::database::DatabaseError, Error,
};

// pub(crate) static mut SQLITE_POOL: once_cell::sync::OnceCell<sqlx::Pool<sqlx::Sqlite>> =
//     once_cell::sync::OnceCell::new();
// pub static SQLITE_POOL: once_cell::sync::Lazy<
//     once_cell::sync::OnceCell<tokio::sync::RwLock<Option<sqlx::Pool<sqlx::Sqlite>>>>,
// > = once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);
// >> = once_cell::sync::Lazy::new(|| tokio::sync::RwLock::new(None));

// pub(crate) fn sqlite_generator<'a>() -> &'a rf_node_sys::Dispatcher {
//     SQLITE_POOL.get_or_init(|| {})
// }

pub fn version() -> std::collections::HashMap<String, String> {
    let mut version = std::collections::HashMap::new();
    version.insert("im-sdk".to_string(), env!("CARGO_PKG_VERSION").to_string());

    version
}

#[cfg(test)]
mod test {
    use crate::db::DbConnection;

    #[test]
    fn version() {
        println!("{:#?}", crate::version());
    }

    #[tokio::test]
    async fn test_sqlite_create() {
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let _ = DbConnection::init_user_database(pri_url.to_string()).await;
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

        let user_id = 123;
        let father_id = Some(234234);
        let bio = "sdfsf".to_string();
        let name = "gffd".to_string();
        let announcement = Some("adsdad".to_string());
        let pinned = true;
        let status = 21;
        let passwd = Some("65765".to_string());
        let res = crate::command::community::info::create_community(
            user_id,
            father_id,
            bio,
            name,
            announcement,
            pinned,
            status,
            passwd,
        )
        .await;
        println!("res: {res:?}");
    }

    #[tokio::test]
    async fn test_sqlite_query() {
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let _ = DbConnection::init_user_database(pri_url.to_string()).await;
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

        let user_id = 123;
        let res = crate::command::community::info::community_list(user_id).await;
        println!("res: {res:#?}");
    }
}
