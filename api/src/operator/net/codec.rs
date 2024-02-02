#[derive(Clone)]
pub(crate) struct CodecChannel(tokio::sync::mpsc::UnboundedSender<im_codec::Protocol>);

impl CodecChannel {
    // wrap channel
    pub(crate) async fn init(
        #[cfg(not(feature = "mock"))] endpoint: std::net::SocketAddr,
        #[cfg(not(feature = "mock"))] hanshake: im_codec::Protocol,
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
        .map_err(|e| crate::SystemError::Net(e))?;
        Ok(Self(res))
    }
    pub(crate) fn send(&self, protocol: im_codec::Protocol) -> Result<(), crate::ChannelError> {
        self.0
            .send(protocol)
            .map_err(|e| crate::ChannelError::SendFailed)
    }

    // init codec
    pub(crate) async fn codec_init(
        #[cfg(not(feature = "mock"))] trace_id: u64,
        #[cfg(not(feature = "mock"))] endpoint: std::net::SocketAddr,
        #[cfg(not(feature = "mock"))] hanshake: im_codec::Protocol,
    ) -> Result<tokio::sync::mpsc::UnboundedSender<im_codec::Protocol>, crate::NetError> {
        use std::{future::Future, pin::Pin};
        // 生成handler
        #[cfg(feature = "mock")]
        let mut handler = {
            let (sink, stream) = tokio::sync::mpsc::unbounded_channel();
            let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(stream);

            let handler = im_codec::Handler::handshake(sink, stream).await?;
            handler
        };
        #[cfg(not(feature = "mock"))]
        let mut handler = {
            let handler =
                im_codec::Handler::handshake(endpoint, im_codec::Protocol::heartbeat(trace_id))
                    .await?;
            handler
        };

        // let sender = super::channel::net_channel_generator();
        let tx = handler.sink();
        // tx.send(protocol)?;

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
        Ok(tx)
    }
}
