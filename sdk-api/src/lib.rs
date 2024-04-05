//
//  Copyright 2024 Ram Flux, LLC.
//

#![feature(try_trait_v2, async_closure, associated_type_defaults, let_chains)]
#![allow(unused)]
#![allow(unreachable_code)]
#![allow(clippy::too_many_arguments)]
pub mod api;
pub mod error;
pub mod handler;
pub mod logic;
pub mod operator;
pub mod resources;
pub mod response;
pub mod service;

pub(crate) use error::{common::channel::ChannelError, common::database::DatabaseError, Error};

pub use operator::sqlite::init::DbConnection;

use error::{common::net::NetError, system::SystemError};

pub fn version() -> std::collections::HashMap<String, String> {
    let mut version = std::collections::HashMap::new();
    version.insert("im-sdk".to_string(), env!("CARGO_PKG_VERSION").to_string());

    version
}

#[allow(dead_code)]
pub(crate) fn init_log() {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
}

// #[cfg(test)]
// mod test {
//     use crate::operator::sqlite::init::DbConnection;

//     #[test]
//     fn version() {
//         println!("{:#?}", crate::version());
//     }

//     #[tokio::test]
//     async fn test_sqlite_create() {
//         let pri_url = "sqlite://test_pri.db";
//         let pub_url = "sqlite://test_pub.db";
//         let _ = DbConnection::init_user_database(pri_url.to_string()).await;
//         let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

//         let user_id = 123;
//         let father_id = Some(234234);
//         let bio = "sdfsf".to_string();
//         let name = "gffd".to_string();
//         let announcement = Some("adsdad".to_string());
//         let pinned = true;
//         let status = 21;
//         let passwd = Some("65765".to_string());
//         let res = crate::api::community::info::create_community(
//             user_id,
//             father_id,
//             bio,
//             name,
//             announcement,
//             pinned,
//             status,
//             passwd,
//         )
//         .await;
//         println!("res: {res:?}");
//     }

//     #[tokio::test]
//     async fn test_sqlite_query() {
//         let pri_url = "sqlite://test_pri.db";
//         let pub_url = "sqlite://test_pub.db";
//         let _ = DbConnection::init_user_database(pri_url.to_string()).await;
//         let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

//         let user_id = 123;
//         let page_size = 3;
//         let offset = 0;
//         let res = crate::api::community::info::community_list(user_id, page_size, offset).await;
//         println!("res: {res:#?}");
//     }

//     #[tokio::test]
//     async fn test_sqlite_query_one() {
//         let pri_url = "sqlite://test_pri.db";
//         let pub_url = "sqlite://test_pub.db";
//         let _ = DbConnection::init_user_database(pri_url.to_string()).await;
//         let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

//         let community_id = 1614942208;
//         let res = crate::api::community::info::community_detail(community_id).await;
//         println!("res: {res:#?}");
//     }
// }
