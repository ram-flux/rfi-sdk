//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) struct SwitchLanguageReq {
    language: payload::resources::settings::language::Language,
    user_id: u32,
}

impl SwitchLanguageReq {
    pub(crate) fn new(
        language: payload::resources::settings::language::Language,
        user_id: u32,
    ) -> Self {
        Self { language, user_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::switch_language(self.language, self.user_id).await?;
        Ok(())
    }
}
