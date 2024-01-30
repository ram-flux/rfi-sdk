use std::{future::Future, pin::Pin};

use im_codec::Protocol;

// pub async fn init_tcp() {
//     im_codec::Handler::handle_handshake(stream)
// }

pub async fn push_msg(
    content: String,
    mode: u8,
    from_id: i32,
    user_id: i32,
    chat_type: u8,
    endpoint: String,
) -> crate::response::Response<()> {
    let mut worker = payload::utils::worker();
    let trace_id = worker.next_id().unwrap();

    #[cfg(feature = "mock")]
    let mut handler = {
        let (sink, stream) = tokio::sync::mpsc::unbounded_channel();
        let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(stream);

        let handler = im_codec::Handler::handshake(sink, stream, Protocol::heartbeat(trace_id))
            .await
            .unwrap();
        handler
    };

    #[cfg(not(feature = "mock"))]
    let mut handler = {
        let handler =
            im_codec::Handler::handshake(endpoint.parse().unwrap(), Protocol::heartbeat(trace_id))
                .await
                .unwrap();
        handler
    };

    let sink = handler.sink();
    let _ = sink.send(Protocol::heartbeat(worker.next_id().unwrap()));

    tokio::spawn(async move {
        let mop = |a: &[[u8; 32]],
                   b: &[u8]|
         -> Pin<Box<dyn Future<Output = Result<(), im_codec::Error>> + Send>> {
            // TODO: 处理message
            Box::pin(async move { Ok::<(), im_codec::Error>(()) })
        };
        let aop = |trace_id: u64,
                   header: im_codec::ResponseHeader|
         -> Pin<Box<dyn Future<Output = Result<(), im_codec::Error>> + Send>> {
            // TODO: 处理response
            Box::pin(async move { Ok::<(), im_codec::Error>(()) })
        };
        if let Err(e) = handler
            .handle_request(|a, b| Box::pin(mop(a, b)), |a, b| Box::pin(aop(a, b)))
            .await
        {
            tracing::error!("[handle_request] error: {e}");
        };
    });

    let message = payload::resources::message::Message {
        datas: content,
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn pull_msg(
    message_id: u32,
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

pub async fn update_msg(
    message_id: u32,
    content: String,
) -> crate::response::Response<payload::resources::message::Message> {
    let message = payload::resources::message::Message {
        datas: content,
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(message).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_msg(message_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn pin_msg(message_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
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
