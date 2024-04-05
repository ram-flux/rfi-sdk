//
//  Copyright 2024 Ram Flux, LLC.
//

#[derive(Debug, serde::Serialize)]
pub struct NewSettingsRes {
    pub(crate) settings_id: u32,
}

impl NewSettingsRes {
    pub async fn new_settings(language: String) -> Result<Self, crate::Error> {
        let settings_id = crate::handler::settings::new_settings(language).await?;
        Ok(Self { settings_id })
    }
}
