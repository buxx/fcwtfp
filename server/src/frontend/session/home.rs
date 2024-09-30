use common::session::SessionKey;
use dioxus::prelude::*;

use crate::{
    api::session::member::get_members,
    frontend::{
        generic::Res,
        session::{member::MembersList, SessionHeader, SessionNavBar},
    },
    RootNavBar,
};

#[component]
pub fn SessionHome(session_key: ReadOnlySignal<String>) -> Element {
    let members = use_resource(move || async move {
        Res::Loaded(get_members(SessionKey(session_key.to_string())).await)
    });

    rsx! {
        RootNavBar {}
        SessionHeader { session_key }
        SessionNavBar { session_key }
        if let Some(Res::Loaded(Ok(members))) = members() {
            MembersList { members: Res::Loaded(members.clone()) }
        }
    }
}
