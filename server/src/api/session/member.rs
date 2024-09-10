use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::backend::{self};
#[cfg(feature = "server")]
use crate::core::session::member::Member;

use crate::core::session::{
    member::{MemberName, Members},
    SessionKey,
};

#[server(GetSessionMembers)]
pub async fn get_members(session_key: SessionKey) -> Result<Members, ServerFnError> {
    Ok(backend::session::member::get_members(&session_key)
        .await
        .unwrap())
}

#[server(DeleteMember)]
pub async fn delete_member(session_key: SessionKey, name: MemberName) -> Result<(), ServerFnError> {
    backend::session::member::remove_member(session_key, Member::builder().name(name).build())
        .await?;
    Ok(())
}

#[server(PostNewMember)]
pub async fn post_new_member(
    session_key: SessionKey,
    name: MemberName,
) -> Result<(), ServerFnError> {
    backend::session::member::add_member(session_key, Member::builder().name(name).build()).await?;
    Ok(())
}
