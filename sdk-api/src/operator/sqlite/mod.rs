//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) mod init;
pub(crate) mod query;

// pub static USER_STATE: once_cell::sync::Lazy<std::sync::Arc<tokio::sync::RwLock<UserState>>> =
//     once_cell::sync::Lazy::new(|| {
//         std::sync::Arc::new(tokio::sync::RwLock::new(UserState::default()))
//     });
pub static USER_STATE: once_cell::sync::Lazy<tokio::sync::RwLock<UserState>> =
    once_cell::sync::Lazy::new(|| tokio::sync::RwLock::new(UserState::default()));

#[derive(Debug, Default)]
pub(crate) struct UserState {
    state: UserData,
}
#[derive(Debug, Default, Clone)]
pub(crate) struct UserData {
    pub(crate) user_id: u32,
}

impl UserState {
    pub(crate) async fn init_user_state(user_id: u32) -> Result<(), crate::SystemError> {
        let mut state = USER_STATE.write().await;
        state.state.user_id = user_id;
        Ok(())
    }

    pub(crate) async fn get_user_state() -> Result<std::sync::Arc<UserData>, crate::SystemError> {
        let mut state = USER_STATE.read().await;
        let state = &state.state;
        Ok(std::sync::Arc::new(state.clone()))
    }
}
