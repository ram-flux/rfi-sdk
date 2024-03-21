#[derive(Debug, serde::Serialize)]
pub struct AddNavRes {
    pub(crate) nav_id: u32,
}

impl AddNavRes {
    pub async fn add_nav(r#type: u8, type_id: u32, sort: u32) -> Result<Self, crate::Error> {
        let nav_id = crate::handler::nav::add_nav(r#type, type_id, sort).await?;
        Ok(Self { nav_id })
    }
}
