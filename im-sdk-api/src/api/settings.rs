/// 新设置(done, untested)
pub async fn new_settings(language: String) -> crate::response::Response<u32> {
    #[cfg(feature = "mock")]
    return Ok(211).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::settings::new_settings(language)
            .await
            .into()
    }
}

/// 设置详情(done, untested)
pub async fn settings_detail() -> crate::response::Response<crate::logic::settings::SettingsDetailRes> {
    #[cfg(feature = "mock")]
    {
        let settings = crate::logic::settings::SettingsDetailRes {
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(settings).into();
    }
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::settings::settings_detail().await.into()
    }
}

/// 切换语言(done, untested)
pub async fn switch_language(language: String) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        crate::handler::settings::switch_language(language)
            .await
            .into()
    }
}
