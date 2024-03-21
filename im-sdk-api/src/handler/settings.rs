/// 新设置(done, untested)
pub async fn new_settings(language: String) -> Result<u32, crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(211);
    #[cfg(not(feature = "mock"))]
    {
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        let settings = payload::resources::settings::Settings::new(user.user_id, language);
        let mut worker = crate::operator::WrapWorker::worker()?;
        let settings_id = worker.gen_id()?;
        crate::service::settings::AddSettings::new(settings, settings_id)
            .exec()
            .await?;
        Ok(settings_id)
    }
}

/// 设置详情(done, untested)
pub async fn settings_detail() -> Result<crate::logic::settings::SettingsDetailRes, crate::Error> {
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
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        Ok(
            crate::service::settings::SettingsDetailReq::new(user.user_id)
                .exec()
                .await?,
        )
    }
}

/// 切换语言(done, untested)
pub async fn switch_language(language: String) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    // #[cfg(not(feature = "mock"))]
    {
        let language = payload::resources::settings::language::Language::new(language);
        let user = crate::operator::sqlite::UserState::get_user_state().await?;
        crate::service::settings::language::SwitchLanguageReq::new(language, user.user_id)
            .exec()
            .await?;

        Ok(())
    }
}
