use dioxus::prelude::*;

use crate::{
    api::session::member::get_members,
    core::session::{member::Members, SessionKey},
    frontend::{
        generic::Res,
        session::{
            member::{add::AddMember, remove::RemoveMember, MembersList},
            SessionHeader, SessionNavBar,
        },
    },
    RootNavBar,
};

#[component]
pub fn MembersHome(session_key: ReadOnlySignal<String>) -> Element {
    let members_ = use_server_future(move || async move {
        Res::Loaded(
            get_members(SessionKey(session_key.to_string()))
                .await
                .unwrap(),
        )
    })?;
    let members = use_signal(|| members_().unwrap_or(Res::Loaded(Members::empty())));

    rsx! {
        RootNavBar {}
        SessionHeader { session_key }
        SessionNavBar { session_key }
        h2 { "Members" }
        MembersList { members }
        h3 { "Add member" }
        AddMember { session_key, members }
        h3 { "Remove member" }
        RemoveMember { session_key, members }
    }
}
