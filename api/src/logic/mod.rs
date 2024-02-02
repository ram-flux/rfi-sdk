pub mod account;
// #[cfg(not(feature = "mock"))]
pub mod message;

#[macro_use]
pub(crate) mod upsert {
    macro_rules! upsert_resource {
        ($fn_name:ident, $resource_type:ident, $action_name:expr, $payload_type:ty) => {
            pub(crate) async fn $fn_name(
                resource: $payload_type,
                id: u32,
            ) -> Result<(), crate::SystemError> {
                // #[cfg(not(feature = "mock"))]
                {
                    use resource::Action as _;
                    let trace_id = crate::operator::WrapWorker::worker()?.gen_trace_id()?;
                    let action = resource::GeneralAction::Upsert {
                        id: Some(id),
                        resource,
                    };
                    let resource_cmd =
                        crate::resources::Resources::$resource_type(resource::Command::new(
                            trace_id,
                            action,
                            stringify!($action_name).to_string(),
                        ));
                    let pool = crate::DbConnection::get_user_connection().await?;
                    let _ = resource_cmd.execute(pool.as_ref()).await;
                }
                Ok(())
            }
        };
    }

    upsert_resource!(
        new_device,
        Device,
        UpsertDevice,
        payload::resources::device::Device
    );

    upsert_resource!(
        new_account,
        Account,
        UpsertAccount,
        payload::resources::account::Account
    );

    upsert_resource!(
        add_community,
        AccountCommunity,
        UpsertAccountCommunity,
        payload::resources::account::community::AccountCommunity
    );

    upsert_resource!(
        new_message,
        Message,
        UpsertMessage,
        payload::resources::message::Message
    );

    upsert_resource!(
        add_contact,
        Contact,
        UpsertContact,
        payload::resources::contact::Contact
    );

    upsert_resource!(
        add_admin,
        Admin,
        UpsertAdmin,
        payload::resources::community::admin::Admin
    );

    upsert_resource!(
        add_member,
        Member,
        UpsertMember,
        payload::resources::community::member::Member
    );

    upsert_resource!(
        create_post,
        Post,
        UpsertPost,
        payload::resources::community::post::Post
    );

    upsert_resource!(
        reply_post,
        PostReply,
        UpsertPostReply,
        payload::resources::community::post_reply::PostReply
    );

    upsert_resource!(
        reply_apply,
        Apply,
        UpsertApply,
        payload::resources::apply::Apply
    );

    upsert_resource!(create_elf, Elf, UpsertElf, payload::resources::elf::Elf);

    upsert_resource!(
        add_favorite,
        Favorite,
        UpsertFavorite,
        payload::resources::favorite::Favorite
    );
    upsert_resource!(add_nav, Nav, UpsertNav, payload::resources::nav::Nav);
    upsert_resource!(
        new_settings,
        Settings,
        UpsertSettings,
        payload::resources::settings::Settings
    );
}

#[macro_use]
pub(crate) mod update {
    macro_rules! update_resource {
        ($fn_name:ident, $resource_type:ident, $action_name:expr, $payload_type:ty) => {
            pub(crate) async fn $fn_name(
                resource: $payload_type,
                id: u32,
            ) -> Result<(), crate::SystemError> {
                // #[cfg(not(feature = "mock"))]
                {
                    use resource::Action as _;
                    let trace_id = crate::operator::WrapWorker::worker()?.gen_trace_id()?;
                    let action = resource::GeneralAction::Update { id, resource };
                    let resource_cmd =
                        crate::resources::Resources::$resource_type(resource::Command::new(
                            trace_id,
                            action,
                            stringify!($action_name).to_string(),
                        ));
                    let pool = crate::DbConnection::get_user_connection().await?;
                    let _ = resource_cmd.execute(pool.as_ref()).await;
                }
                Ok(())
            }
        };
    }

    update_resource!(
        update_account,
        Account,
        UpdateAccount,
        payload::resources::account::Account
    );
    update_resource!(
        update_token,
        Token,
        UpdateToken,
        payload::resources::device::token::Token
    );
    update_resource!(
        update_avatar,
        Avatar,
        UpdateAvatar,
        payload::resources::account::avatar::Avatar
    );
}

#[macro_use]
pub(crate) mod delete {
    macro_rules! delete_resource {
        ($fn_name:ident, $resource_type:ident, $action_name:expr) => {
            pub(crate) async fn $fn_name(id: u32) -> Result<(), crate::SystemError> {
                // #[cfg(not(feature = "mock"))]
                {
                    use resource::Action as _;
                    let trace_id = crate::operator::WrapWorker::worker()?.gen_trace_id()?;
                    let action = resource::GeneralAction::Drop(id);
                    let resource_cmd =
                        crate::resources::Resources::$resource_type(resource::Command::new(
                            trace_id,
                            action,
                            stringify!($action_name).to_string(),
                        ));
                    let pool = crate::DbConnection::get_user_connection().await?;
                    let _ = resource_cmd.execute(pool.as_ref()).await;
                }
                Ok(())
            }
        };
    }

    delete_resource!(del_device, Device, DropDevice);
    delete_resource!(quit_community, AccountCommunity, DropAccountCommunity);
}
