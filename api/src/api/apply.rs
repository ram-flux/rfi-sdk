pub async fn apply_list(
    user_id: u32,
    page_size: u16,
    offset: u16,
) -> Result<
    crate::operator::sqlite::query::QueryResult<crate::logic::apply::ApplyDetailRes>,
    crate::Error,
> {
    #[cfg(feature = "mock")]
    {
        let list = vec![
            crate::logic::apply::ApplyDetailRes {
                user_id: 6546,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
            crate::logic::apply::ApplyDetailRes {
                user_id: 5435,
                created_at: payload::utils::time::now(),
                updated_at: Some(payload::utils::time::now()),
                ..Default::default()
            },
        ];
        return Ok(crate::operator::sqlite::query::QueryResult::Vec(list));
    }
    #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(
            crate::service::apply::ApplyListReq::new(1, 2, page_size, offset)
                .exec()
                .await?,
        )
    }
}

pub async fn apply_detail(
    apply_id: u32,
) -> Result<
    crate::operator::sqlite::query::QueryResult<crate::logic::apply::ApplyDetailRes>,
    crate::Error,
> {
    #[cfg(feature = "mock")]
    {
        let apply = crate::logic::apply::ApplyDetailRes {
            r#type: 1,
            user_id: 6565656,
            created_at: payload::utils::time::now(),
            updated_at: Some(payload::utils::time::now()),
            ..Default::default()
        };
        return Ok(crate::operator::sqlite::query::QueryResult::One(apply));
    }
    #[cfg(not(feature = "mock"))]
    {
        use crate::operator::sqlite::query::Query;
        Ok(crate::service::apply::ApplyDetailReq::new(apply_id)
            .exec()
            .await?)
    }
}

pub async fn reply_apply(
    apply_id: u32,
    user_id: u32,
    content: String,
    status: u8,
) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_apply(apply_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

pub async fn del_apply_reply(apply_id: u32) -> Result<(), crate::Error> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}
