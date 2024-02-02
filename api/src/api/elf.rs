pub async fn create_elf(r#type: u8, name: String, avatar: String) -> Result<(), crate::Error> {
    let elf = payload::resources::elf::Elf {
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

pub async fn update_elf(
    elf_id: u32,
    r#type: u8,
    name: String,
    avatar: String,
    status: u8,
) -> Result<(), crate::Error> {
    let elf = payload::resources::elf::Elf {
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

pub async fn elf_detail(elf_id: u32) -> Result<payload::resources::elf::Elf, crate::Error> {
    let elf = payload::resources::elf::Elf {
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(elf).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_elf(elf_id: u32) -> Result<(), crate::Error> {
    let elf = payload::resources::elf::Elf {
        updated_at: Some(payload::utils::time::now()),
        ..Default::default()
    };
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
