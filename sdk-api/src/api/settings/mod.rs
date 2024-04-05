//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod param;
/// 新设置(done, untested)
pub async fn new_settings(language: String) -> crate::response::Response<param::NewSettingsRes> {
    param::NewSettingsRes::new_settings(language).await.into()
}

/// 设置详情(done, untested)
pub async fn settings_detail(
) -> crate::response::Response<crate::logic::settings::SettingsDetailRes> {
    crate::handler::settings::settings_detail().await.into()
}

/// 切换语言(done, untested)
pub async fn switch_language(language: String) -> crate::response::Response<()> {
    crate::handler::settings::switch_language(language)
        .await
        .into()
}
