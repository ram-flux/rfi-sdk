//
//  Copyright 2024 Ram Flux, LLC.
//

pub mod account;
pub mod apply;
pub mod chat;
pub mod community;
pub mod contact;
pub mod elf;
pub mod favorite;
pub mod nav;
pub mod settings;
// #[cfg(not(feature = "mock"))]
pub mod message;

#[macro_use]
pub(crate) mod upsert {
    macro_rules! upsert_resource {
        ($fn_name:ident, $resource_type:ident, $action_name:expr, $payload_type:ty, $id_ty: ty) => {
            pub(crate) async fn $fn_name(
                resource: $payload_type,
                // id: u32,
                id: $id_ty,
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
        payload::resources::device::Device,
        u32
    );

    upsert_resource!(
        new_account,
        Account,
        UpsertAccount,
        payload::resources::account::Account,
        u32
    );

    upsert_resource!(
        join_community,
        AccountCommunity,
        UpsertAccountCommunity,
        payload::resources::account::community::AccountCommunity,
        u32
    );

    upsert_resource!(
        create_account_elf,
        AccountElf,
        UpsertAccountElf,
        payload::resources::account::elf::AccountElf,
        u32
    );

    upsert_resource!(
        new_message,
        Message,
        UpsertMessage,
        payload::resources::message::Message,
        u32
    );

    upsert_resource!(
        add_contact,
        Contact,
        UpsertContact,
        payload::resources::contact::Contact,
        u32
    );

    upsert_resource!(
        create_community,
        Community,
        UpsertCommunity,
        payload::resources::community::Community,
        u32
    );

    upsert_resource!(
        add_admin,
        CommunityAdmin,
        UpsertAdmin,
        payload::resources::community::admin::CommunityAdmin,
        u32
    );

    upsert_resource!(
        add_member,
        CommunityMember,
        UpsertMember,
        payload::resources::community::member::CommunityMember,
        (u32, u32) // u32
    );

    upsert_resource!(
        create_post,
        Post,
        UpsertPost,
        payload::resources::community::post::Post,
        u32
    );

    upsert_resource!(
        reply_post,
        PostReply,
        UpsertPostReply,
        payload::resources::community::post_reply::PostReply,
        u32
    );

    upsert_resource!(
        create_apply,
        Apply,
        UpsertApply,
        payload::resources::apply::Apply,
        u32
    );

    upsert_resource!(
        reply_apply,
        ApplyReply,
        UpsertApplyReply,
        payload::resources::apply::reply::ApplyReply,
        u32
    );

    upsert_resource!(
        create_elf,
        Elf,
        UpsertElf,
        payload::resources::elf::Elf,
        u32
    );

    upsert_resource!(
        add_favorite,
        Favorite,
        UpsertFavorite,
        payload::resources::favorite::Favorite,
        u32
    );
    upsert_resource!(add_nav, Nav, UpsertNav, payload::resources::nav::Nav, u32);
    upsert_resource!(
        new_settings,
        Settings,
        UpsertSettings,
        payload::resources::settings::Settings,
        u32
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
        update_admin_type,
        CommunityAdminType,
        UpdateAdmin,
        payload::resources::community::admin::typ::CommunityAdminType
    );
    // update_resource!(
    //     update_member,
    //     CommunityMember,
    //     UpdateMember,
    //     payload::resources::community::member::CommunityMember
    // );
    update_resource!(
        update_member_type,
        CommunityMemberType,
        UpdateMember,
        payload::resources::community::member::typ::CommunityMemberType
    );
    update_resource!(
        update_post,
        Post,
        UpdatePost,
        payload::resources::community::post::Post
    );
    update_resource!(
        edit_post,
        PostInfo,
        EditPost,
        payload::resources::community::post::info::PostInfo
    );
    // update_resource!(
    //     update_post_reply,
    //     PostReply,
    //     UpdatePost,
    //     payload::resources::community::post_reply::PostReply
    // );
    update_resource!(
        edit_post_reply,
        PostReplyInfo,
        EditPostReply,
        payload::resources::community::post_reply::info::PostReplyInfo
    );

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
        AccountAvatar,
        UpdateAvatar,
        payload::resources::account::avatar::AccountAvatar
    );
    update_resource!(
        update_chat_status,
        ChatStatus,
        UpdateChatStatus,
        payload::resources::chat::status::ChatStatus
    );
    update_resource!(
        update_contact,
        Contact,
        UpdateContact,
        payload::resources::contact::Contact
    );
    update_resource!(
        update_contact_remark,
        ContactRemark,
        UpdateContactRemark,
        payload::resources::contact::remark::ContactRemark
    );
    update_resource!(
        update_community,
        CommunityInfo,
        UpdateCommunityInfo,
        payload::resources::community::info::CommunityInfo
    );
    update_resource!(update_elf, Elf, UpdateElf, payload::resources::elf::Elf);
    update_resource!(
        update_message,
        Message,
        UpdateMessage,
        payload::resources::message::Message
    );
    update_resource!(
        update_message_status,
        Status,
        UpdateMessageStatus,
        payload::resources::message::status::MessageStatus
    );
    update_resource!(
        edit_message,
        Status,
        UpdateMessageStatus,
        payload::resources::message::status::MessageStatus
    );
    update_resource!(update_nav, Nav, UpdateNav, payload::resources::nav::Nav);
    update_resource!(
        switch_language,
        Language,
        UpdateLanguage,
        payload::resources::settings::language::Language
    );
}

#[macro_use]
pub(crate) mod delete {
    macro_rules! delete_resource {
        ($fn_name:ident, $resource_type:ident, $action_name:expr, $id_ty: ty) => {
            pub(crate) async fn $fn_name(id: $id_ty) -> Result<(), crate::SystemError> {
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

    delete_resource!(del_admin, CommunityAdmin, DropAdmin, u32);
    delete_resource!(del_community, Community, DropCommunity, u32);
    delete_resource!(del_member, CommunityMember, DropMember, (u32, u32));
    delete_resource!(del_post, Post, DropPost, u32);
    delete_resource!(del_post_reply, PostReply, DropPostReply, u32);
    delete_resource!(del_apply, Apply, DropApply, u32);
    delete_resource!(del_apply_reply, ApplyReply, DropApplyReply, u32);
    delete_resource!(del_contact, Contact, DropContact, u32);
    delete_resource!(del_device, Device, DropDevice, u32);
    delete_resource!(del_elf, Elf, DropElf, u32);
    delete_resource!(del_favorite, Favorite, DropFavorite, u32);
    delete_resource!(del_message, Message, DropMessage, u32);
    delete_resource!(del_nav, Nav, DropNav, u32);
    delete_resource!(quit_community, AccountCommunity, DropAccountCommunity, u32);
}
