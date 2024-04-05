//
//  Copyright 2024 Ram Flux, LLC.
//

#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    #[error("Channel send failed")]
    SendFailed,
}

impl ChannelError {
    pub fn get_status_code(&self) -> u32 {
        match self {
            ChannelError::SendFailed => 6300,
        }
    }
}
