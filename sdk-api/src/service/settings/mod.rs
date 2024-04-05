//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) mod language;
pub(crate) struct AddSettings {
    settings: payload::resources::settings::Settings,
    settings_id: u32,
}

impl AddSettings {
    pub(crate) fn new(settings: payload::resources::settings::Settings, settings_id: u32) -> Self {
        Self {
            settings,
            settings_id,
        }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::new_settings(self.settings, self.settings_id).await?;
        Ok(())
    }
}

pub(crate) struct SettingsDetailReq {
    user_id: u32,
}

impl SettingsDetailReq {
    pub(crate) fn new(user_id: u32) -> Self {
        Self { user_id }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<crate::logic::settings::SettingsDetailRes, crate::SystemError> {
        crate::logic::settings::SettingsDetailRes::detail(self.user_id).await
    }
}
