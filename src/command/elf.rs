pub async fn create(r#type: u8, name: String, avatar: String) -> Result<(), crate::Error> {
    let elf = payload::resources::elf::Elf {
        id: 123123,
        r#type,
        name,
        avatar,
        status: 1,
        created_at: payload::utils::time::now(),
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn update(
    elf_id: u32,
    r#type: u8,
    name: String,
    avatar: String,
    status: u8,
) -> Result<(), crate::Error> {
    let elf = payload::resources::elf::Elf {
        id: elf_id,
        r#type,
        name,
        avatar,
        status,
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
