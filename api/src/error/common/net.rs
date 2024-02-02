#[derive(Debug, thiserror::Error)]
pub enum NetError {
    #[error("Codec error: {0}")]
    Codec(#[from] im_net::Error),
    #[error("Channel error: {0}")]
    Channel(#[from] super::channel::ChannelError),
    #[error("Parse error: {0}")]
    Parse(#[from] super::parse::ParseError),
}

impl NetError {
    pub fn get_status_code(&self) -> u32 {
        match self {
            NetError::Codec(_) => 6300,
            NetError::Channel(_) => 6300,
            NetError::Parse(_) => 6300,
        }
    }
}
