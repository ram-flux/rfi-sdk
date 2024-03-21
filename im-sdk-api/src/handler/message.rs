/// 发送消息(tested)
pub async fn push_msg(
    content: String,
    mode: u8,
    from_id: u32,
    user_id: u32,
    chat_id: u32,
    chat_type: u8,
    _endpoint: String,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();

    #[cfg(not(feature = "mock"))]
    {
        let mut worker = crate::operator::WrapWorker::worker()?;
        let message_id = worker.gen_id()?;
        let message =
            payload::resources::message::Message::new(from_id, user_id, chat_id, chat_type)
                .set_data(&content, mode);
        let recv_list = Vec::new();
        crate::service::message::SendMessageReq::new(message, message_id, recv_list)
            .exec()
            .await?;
        Ok(())
    }
}

pub async fn pull_msg(
    _message_id: u32,
) -> Result<Vec<payload::resources::message::Message>, crate::Error> {
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
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    {
        return Ok(());
    }
    #[cfg(not(feature = "mock"))]
    {
        let message =
            payload::resources::message::Message::new(from_id, user_id, chat_id, chat_type)
                .set_data(&content, mode);
        crate::service::message::UpdateMessageReq::new(message, message_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 删除消息(done, untested)
pub async fn del_msg(message_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        crate::service::message::DeleteMessageReq::new(message_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 置顶消息(done, untested)
pub async fn pin_msg(message_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let message_status = payload::resources::message::status::MessageStatus::new(5);
        crate::service::message::status::UpdateMessageStatusReq::new(message_status, message_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 取消置顶消息(done, untested)
pub async fn unpin_msg(message_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        let message_status = payload::resources::message::status::MessageStatus::new(2);
        crate::service::message::status::UpdateMessageStatusReq::new(message_status, message_id)
            .exec()
            .await?;
        Ok(())
    }
}

/// 撤回消息(done, untested)
pub async fn revoke_msg(message_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        let message_status = payload::resources::message::status::MessageStatus::new(4);
        crate::service::message::status::UpdateMessageStatusReq::new(message_status, message_id)
            .exec()
            .await?;
        Ok(())
    }
}

// struct Test {}

// impl Test {
//     pub async fn handle_request<MF, AF, U>(
//         &mut self,
//         message_op: MF,
//         resp_op: AF,
//     ) -> Result<(), crate::Error>
//     where
//         U: std::future::Future<Output = Result<(), crate::Error>>,
//         MF: Fn(i8, i8) -> U,
//         AF: Fn(u64 /* trace */, u64) -> U,
//     {
//         message_op(1, 1).await?;
//         resp_op(1, 1).await?;
//         Ok(())
//     }
// }

// async fn a() {
//     let op = |a: i8, b: i8| -> Pin<Box<dyn Future<Output = Result<(), crate::Error>>>> {
//         Box::pin(async move { Ok::<(), crate::Error>(()) })
//     };
//     let aop = |a: u64, b: u64| -> Pin<Box<dyn Future<Output = Result<(), crate::Error>>>> {
//         Box::pin(async move { Ok::<(), crate::Error>(()) })
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
