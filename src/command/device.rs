pub async fn init_device(
    account_pk: String,
    device_pk: String,
    proof: String,
) -> crate::response::Response<String> {
    let device = payload::resources::device::Device::default();
    #[cfg(feature = "mock")]
    return Ok("encrypt_data".to_string()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

// 授权
pub async fn warrant() -> crate::response::Response<String> {
    #[cfg(feature = "mock")]
    return Ok("server_pubkey".to_string()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update_token() -> crate::response::Response<String> {
    let device = payload::resources::device::Device::default();
    let token = device.token;
    #[cfg(feature = "mock")]
    return Ok(token).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_device(device_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
