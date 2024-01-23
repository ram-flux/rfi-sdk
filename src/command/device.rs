pub async fn init_device(
    account_pk: String,
    device_pk: String,
    proof: String,
) -> crate::response::Response<payload::resources::device::Device> {
    let device = payload::resources::device::Device::default();
    #[cfg(feature = "mock")]
    return Ok(device).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

// 授权
pub async fn warrant() -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn token() -> crate::response::Response<String> {
    let device = payload::resources::device::Device::default();
    let token = device.token;
    #[cfg(feature = "mock")]
    return Ok(token).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del() -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
