//
//  Copyright 2024 Ram Flux, LLC.
//

pub(crate) struct AddNav {
    nav: payload::resources::nav::Nav,
    nav_id: u32,
}

impl AddNav {
    pub(crate) fn new(nav: payload::resources::nav::Nav, nav_id: u32) -> Self {
        Self { nav, nav_id }
    }

    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::upsert::add_nav(self.nav, self.nav_id).await?;
        Ok(())
    }
}

pub(crate) struct UpdateNav {
    nav: payload::resources::nav::Nav,
    nav_id: u32,
}

impl UpdateNav {
    pub(crate) fn new(nav: payload::resources::nav::Nav, nav_id: u32) -> Self {
        Self { nav, nav_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::update::update_nav(self.nav, self.nav_id).await?;
        Ok(())
    }
}

pub(crate) struct NavListReq {
    user_id: u32,
    page_size: u16,
    offset: u16,
}

impl NavListReq {
    pub(crate) fn new(user_id: u32, page_size: u16, offset: u16) -> Self {
        Self {
            user_id,
            page_size,
            offset,
        }
    }
    pub(crate) async fn exec(
        self,
    ) -> Result<Vec<crate::logic::nav::NavDetailRes>, crate::SystemError> {
        crate::logic::nav::NavDetailRes::list(self.user_id, self.page_size, self.offset).await
    }
}

pub(crate) struct DeleteNavReq {
    nav_id: u32,
}

impl DeleteNavReq {
    pub(crate) fn new(nav_id: u32) -> Self {
        Self { nav_id }
    }
    pub(crate) async fn exec(self) -> Result<(), crate::SystemError> {
        crate::logic::delete::del_nav(self.nav_id).await?;
        Ok(())
    }
}
