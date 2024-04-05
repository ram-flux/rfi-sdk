//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) struct AddFavorite {
    favorite: payload::resources::favorite::Favorite,
    favorite_id: u32,
}

impl AddFavorite {
    pub(crate) fn new(favorite: payload::resources::favorite::Favorite, favorite_id: u32) -> Self {
        Self {
            favorite,
            favorite_id,
        }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::add_favorite(self.favorite, self.favorite_id).await?;
        Ok(())
    }
}

// pub(crate) struct UpdateElfReq {
//     elf: payload::resources::elf::Elf,
//     elf_id: u32,
// }

// impl UpdateElfReq {
//     pub(crate) fn new(elf: payload::resources::elf::Elf, elf_id: u32) -> Self {
//         Self { elf, elf_id }
//     }
//     pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
//         crate::logic::update::update_elf(self.elf, self.elf_id).await?;
//         Ok(())
//     }
// }

pub(crate) struct FavoriteDetailReq {
    favorite_id: u32,
}

impl FavoriteDetailReq {
    pub(crate) fn new(favorite_id: u32) -> Self {
        Self { favorite_id }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<crate::logic::favorite::FavoriteDetailRes, crate::SystemError> {
        crate::logic::favorite::FavoriteDetailRes::detail(self.favorite_id).await
    }
}

pub(crate) struct FavoriteListReq {
    user_id: u32,
    page_size: u16,
    offset: u16,
}

impl FavoriteListReq {
    pub(crate) fn new(user_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            user_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<Vec<crate::logic::favorite::FavoriteDetailRes>, crate::SystemError> {
        crate::logic::favorite::FavoriteDetailRes::list(self.user_id, self.page_size, self.offset)
            .await
    }
}

pub(crate) struct DeleteFavoriteReq {
    favorite_id: u32,
}

impl DeleteFavoriteReq {
    pub(crate) fn new(favorite_id: u32) -> Self {
        Self { favorite_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_favorite(self.favorite_id).await?;
        Ok(())
    }
}
