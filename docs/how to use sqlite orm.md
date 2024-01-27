# How to use sqlite orm



## Insert|Update|Upsert

以社区举例

```rust
use chrono::prelude::*;

use resource::{GenResourceID, Resource};
use sqlx::Sqlite;

pub mod admin;
pub mod member;
pub mod post;
pub mod post_reply;

#[derive(
    serde::Deserialize,
    serde::Serialize,
    PartialEq,
    Debug,
    resource::resource_macros::Resource,
    Default,
    sqlx::FromRow,
)]
// resource宏帮助自动为这个结构体实现insert、update、upsert方法
#[resource(
    schema_name = "public",
    pg_table_name = "community",
    sqlite_table_name = "community",
  	// 指定主键
    primary_key = "id:u32",
  	// 指定约束
    constraint = "im_community_id_idx"
)]
pub struct Community {
    pub father_id: Option<u32>,
    pub user_id: u32,
    pub name: String,
    pub bio: String,
    pub passwd: Option<String>,
    pub announcement: Option<String>,
    pub pinned: bool,
    pub status: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Community {
    pub fn new(
        father_id: Option<u32>,
        user_id: u32,
        name: String,
        bio: String,
        passwd: Option<String>,
        announcement: Option<String>,
        pinned: bool,
        status: u8,
    ) -> Self {
        Self {
            father_id,
            user_id,
            name,
            bio,
            passwd,
            announcement,
            pinned,
            status,
            created_at: crate::utils::time::now(),
            updated_at: Some(crate::utils::time::now()),
        }
    }
}

// 这里产生符合主键约束的id
impl resource::GenResourceID for Community {
    type Target = u32;

    async fn gen_id() -> Result<Self::Target, resource::Error> {
        let mut id_worker = crate::utils::snowflake::SnowflakeIdWorkerInner::new(1, 1).unwrap();
        let id1 = id_worker.next_id().unwrap();
        Ok(id1 as u32)
    }
}

```





## Query

- user_pool：用户数据库连接池，用于操作用户的sqlite文件，存放不同用户的私有数据
- pub_pool：公共数据库连接池，用于操作公共的sqlite文件，存放元数据

> 需要查询的数据结构的字段要与Select子句的元素对应

### 分页查询

以社区列表举例

```rust
// 函数体：

// {需要查询的数据结构}::query(async move |{用户数据库连接池}, {公共数据库连接池}|{
Community::query(async move |user_pool, pub_pool| {
  			//	sql语句
        let sql = "SELECT id, father_id, user_id, name, bio, passwd, announcement,
                 pinned, status, created_at, updated_at 
            FROM community 
            WHERE user_id = $1
  			//  分页			偏移
            LIMIT $2 OFFSET $3;";

        sqlx::query_as::<sqlx::Sqlite, Community>(sql)
  					//	占位符1
            .bind(user_id)
    				//	占位符2
            .bind(page_size)
    				//	占位符3
            .bind(offset)
  					// 查询所有
            .fetch_all(user_pool.as_ref())
            .await
            .map(Into::into)
    })
    .await
    .map_err(|e| crate::Error::BadRequest(crate::CommunityError::DatabaseError(e).into()))
    .into()
```



### 查询一条

以社区详情举例

```rust
// 函数体：

// {需要查询的数据结构}::query(async move |{用户数据库连接池}, {公共数据库连接池}|{
Community::query(async move |user_pool, pub_pool| {
            let sql =
                "SELECT id, father_id, user_id, name, bio, passwd, announcement,
                    pinned, status, created_at, updated_at 
                FROM community 
                WHERE id =$1;";
            sqlx::query_as::<sqlx::Sqlite, Community>(sql)
    						//	占位符1
                .bind(community_id)
  							// 	查询一条
                .fetch_one(user_pool.as_ref())
                .await
                .map(Into::into)
        })
        .await
        .map_err(|e| crate::Error::BadRequest(crate::CommunityError::DatabaseError(e).into()))
        .into()
```



