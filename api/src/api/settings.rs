pub async fn settings_detail(
    user_id: u32,
) -> Result<payload::resources::settings::Settings, crate::Error> {
    let elf = payload::resources::settings::Settings {
        user_id,
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(elf).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update_language(user_id: u32, language: String) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
