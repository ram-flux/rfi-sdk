use std::{future::Future, pin::Pin};
pub(crate) async fn send_message(
    message: payload::resources::message::Message,
    message_id: u32,
) -> Result<(), crate::Error> {
    // #[cfg(feature = "mock")]
    let mut worker = payload::utils::worker();
    let trace_id = worker.next_id().unwrap();

    // 生成handler

    #[cfg(feature = "mock")]
    let mut handler = {
        let (sink, stream) = tokio::sync::mpsc::unbounded_channel();
        let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(stream);

        let handler = im_codec::Handler::handshake(sink, stream).await.unwrap();
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
    let protocol = im_codec::Protocol::heartbeat(worker.next_id().unwrap());
    tracing::info!("[push_msg] protocol: {protocol:?}");

    let _ = sink.send(protocol);

    tokio::spawn(async move {
        let mop = |a: Vec<[u8; 32]>,
                   b: std::sync::Arc<Vec<u8>>|
         -> Pin<Box<dyn Future<Output = Result<(), im_codec::Error>> + Send>> {
            // TODO: 处理message
            tracing::error!("[handle_request] message a: {a:?}, b: {b:?}");
            Box::pin(async move { Ok::<(), im_codec::Error>(()) })
        };
        let aop = |trace_id: u64,
                   header: im_codec::ResponseHeader|
         -> Pin<Box<dyn Future<Output = Result<(), im_codec::Error>> + Send>> {
            // TODO: 处理response
            tracing::error!("[handle_request] response trace_id: {trace_id:?}, header: {header:?}");
            Box::pin(async move { Ok::<(), im_codec::Error>(()) })
        };
        if let Err(e) = handler
            .handle_request(|a, b| Box::pin(mop(a, b)), |a, b| Box::pin(aop(a, b)))
            .await
        {
            tracing::error!("[handle_request] error: {e}");
        };
    });

    Ok(())
}
