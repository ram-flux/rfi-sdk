#[derive(Clone)]
pub(crate) struct CodecChannel(
    #[allow(dead_code)] tokio::sync::mpsc::UnboundedSender<im_net::Packet>,
);

impl CodecChannel {
    // wrap channel
    pub(crate) async fn init(
        #[cfg(not(feature = "mock"))] endpoint: std::net::SocketAddr,
        #[cfg(not(feature = "mock"))] hanshake: im_net::Packet,
    ) -> Result<Self, crate::SystemError> {
        let trace_id = crate::operator::WrapWorker::worker()?.gen_trace_id()? as u64;
        let res = CodecChannel::codec_init(
            #[cfg(not(feature = "mock"))]
            trace_id,
            #[cfg(not(feature = "mock"))]
            endpoint,
            #[cfg(not(feature = "mock"))]
            hanshake,
        )
        .await
        .map_err(crate::SystemError::Net)?;
        Ok(Self(res))
    }

    // #[cfg(not(feature = "mock"))]
    pub(crate) fn send(&self, protocol: im_net::Packet) -> Result<(), crate::ChannelError> {
        self.0
            .send(protocol)
            .map_err(|e| crate::ChannelError::SendFailed)
    }

    // init codec
    pub(crate) async fn codec_init(
        #[cfg(not(feature = "mock"))] trace_id: u64,
        #[cfg(not(feature = "mock"))] endpoint: std::net::SocketAddr,
        #[cfg(not(feature = "mock"))] hanshake: im_net::Packet,
    ) -> Result<tokio::sync::mpsc::UnboundedSender<im_net::Packet>, crate::NetError> {
        use std::{future::Future, pin::Pin};
        // 生成handler
        let mut handler = {
            #[cfg(feature = "mock")]
            let (sink, stream) = tokio::sync::mpsc::unbounded_channel();
            #[cfg(feature = "mock")]
            let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(stream);

            im_net::Handler::handshake(
                #[cfg(feature = "mock")]
                sink,
                #[cfg(feature = "mock")]
                stream,
                #[cfg(not(feature = "mock"))]
                endpoint,
                #[cfg(not(feature = "mock"))]
                im_net::Packet::heartbeat(trace_id),
                10,
                5,
                3,
            )
            .await?
        };

        // let sender = super::channel::net_channel_generator();
        let tx = handler.sink();
        // tx.send(protocol)?;

        tokio::spawn(async move {
            let mop = |a: Vec<[u8; 32]>,
               b: std::sync::Arc<Vec<u8>>|
            -> Pin<Box<dyn Future<Output = Result<(), im_net::Error>> + Send>> {
                // TODO: 处理message
                tracing::error!("[handle_request] message a: {a:?}, b: {b:?}");
                Box::pin(async move { Ok::<(), im_net::Error>(()) })
            };
            let aop = |trace_id: u64,
                    header: im_net::ResponseHeader|
            -> Pin<Box<dyn Future<Output = Result<(), im_net::Error>> + Send>> {
                // TODO: 处理response
                tracing::error!("[handle_request] response trace_id: {trace_id:?}, header: {header:?}");
                Box::pin(async move { Ok::<(), im_net::Error>(()) })
            };
            if let Err(e) = handler
                .handle_request(
                    |a, b| Box::pin(mop(a, b)),
                    #[cfg(feature = "mock")]
                    // |a, b| Box::pin(aop(a, b))
                    true,
                )
                .await
            {
                tracing::error!("[handle_request] error: {e}");
            };
        });
        Ok(tx)
    }
}
