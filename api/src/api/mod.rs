pub mod account;
pub mod apply;
pub mod chat;
pub mod community;
pub mod contact;
pub mod device;
pub mod elf;
pub mod favorite;
pub mod message;
pub mod nav;
pub mod settings;

pub async fn init_codec(
    from_id: u32,
    recv_id: u32,
    #[cfg(not(feature = "mock"))] endpoint: std::net::SocketAddr,
    #[cfg(not(feature = "mock"))] hanshake: im_net::Protocol,
) -> Result<(), crate::Error> {
    // #[cfg(not(feature = "mock"))]
    // todo!()

    let net_tx = crate::operator::net::channel::net_channel_generator();
    let sender = crate::operator::net::codec::CodecChannel::init(
        #[cfg(not(feature = "mock"))]
        endpoint,
        #[cfg(not(feature = "mock"))]
        hanshake,
    )
    .await
    .map_err(crate::Error::System)?;

    net_tx
        .send(crate::operator::net::channel::Event::Join {
            from_id,
            recv_id,
            sender,
        })
        .map_err(|e| crate::SystemError::Net(e.into()).into())
}
