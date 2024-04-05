//
//  Copyright 2024 Ram Flux, LLC.
//

#[derive(Debug, serde::Serialize)]
pub struct AddFavoriteRes {
    pub(crate) favorite_id: u32,
}

impl AddFavoriteRes {
    pub async fn add_favorite(content: String) -> Result<Self, crate::Error> {
        let favorite_id = crate::handler::favorite::add_favorite(content).await?;
        Ok(Self { favorite_id })
    }
}
