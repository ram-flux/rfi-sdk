pub mod param;
/// 发送消息(tested)
pub async fn push_msg(
    content: String,
    mode: u8,
    from_id: u32,
    user_id: u32,
    chat_id: u32,
    chat_type: u8,
    _endpoint: String,
) -> crate::response::Response<param::PushMsgRes> {
    param::PushMsgRes::push_msg(
        content,
        mode,
        from_id,
        user_id,
        chat_id,
        chat_type,
        "endpoint".to_string(),
    )
    .await
    .into()
}

pub async fn pull_msg(
    _message_id: u32,
) -> crate::response::Response<Vec<payload::resources::message::Message>> {
    #[cfg(feature = "mock")]
    {
        let msgs = vec![
            payload::resources::message::Message {
                datas: "asdsad".to_string(),
                mode: 1,
                ..Default::default()
            },
            payload::resources::message::Message {
                datas: "fsdfs".to_string(),
                mode: 1,
                ..Default::default()
            },
        ];
        return Ok(msgs).into();
    }

    #[cfg(not(feature = "mock"))]
    todo!()
}

/// 更新消息(done, untested)
pub async fn update_msg(
    from_id: u32,
    user_id: u32,
    chat_id: u32,
    chat_type: u8,
    mode: u8,
    message_id: u32,
    content: String,
) -> crate::response::Response<()> {
    crate::handler::message::update_msg(
        from_id, user_id, chat_id, chat_type, mode, message_id, content,
    )
    .await
    .into()
}

/// 删除消息(done, untested)
pub async fn del_msg(message_id: u32) -> crate::response::Response<()> {
    crate::handler::message::del_msg(message_id).await.into()
}

/// 置顶消息(done, untested)
pub async fn pin_msg(message_id: u32) -> crate::response::Response<()> {
    crate::handler::message::pin_msg(message_id).await.into()
}

/// 取消置顶消息(done, untested)
pub async fn unpin_msg(message_id: u32) -> crate::response::Response<()> {
    crate::handler::message::unpin_msg(message_id).await.into()
}

/// 撤回消息(done, untested)
pub async fn revoke_msg(message_id: u32) -> crate::response::Response<()> {
    crate::handler::message::revoke_msg(message_id).await.into()
}

// struct Test {}

// impl Test {
//     pub async fn handle_request<MF, AF, U>(
//         &mut self,
//         message_op: MF,
//         resp_op: AF,
//     ) -> crate::response::Response<()>
//     where
//         U: std::future::Future<Output = crate::response::Response<()>>,
//         MF: Fn(i8, i8) -> U,
//         AF: Fn(u64 /* trace */, u64) -> U,
//     {
//         message_op(1, 1).await?;
//         resp_op(1, 1).await?;
//         Ok(())
//     }
// }

// async fn a() {
//     let op = |a: i8, b: i8| -> Pin<Box<dyn Future<Output = crate::response::Response<()>>>> {
//         Box::pin(async move { Ok::<()>(()) })
//     };
//     let aop = |a: u64, b: u64| -> Pin<Box<dyn Future<Output = crate::response::Response<()>>>> {
//         Box::pin(async move { Ok::<()>(()) })
//     };

//     let mut t = Test {};
//     // t.handle_request(op, aop).await;
//     t.handle_request(|a, b| Box::pin(op(a, b)), |a, b| Box::pin(aop(a, b)))
//         .await;
// }

#[cfg(test)]
mod test {
    use crate::operator::sqlite::init::DbConnection;

    #[tokio::test]
    async fn test_push_message() {
        crate::init_log();
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let _ = DbConnection::init_user_database(pri_url.to_string()).await;
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

        let content = "234234".to_string();
        let mode = 1;
        let from_id = 1221;
        let user_id = 2122;
        let chat_id = 2122;
        let chat_type = 3;
        let endpoint = "avatar".to_string();
        let res = crate::api::message::push_msg(
            content, mode, from_id, user_id, chat_id, chat_type, endpoint,
        )
        .await;
        println!("res: {res:?}");
    }

    #[tokio::test]
    async fn test_update_avatar() {
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let _ = DbConnection::init_user_database(pri_url.to_string()).await;
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;

        let user_id = 123;
        let avatar = "dfdsfdsf".to_string();
        let res = crate::api::account::update_avatar(user_id, avatar).await;
        println!("res: {res:?}");
    }
}
