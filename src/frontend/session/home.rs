use dioxus::prelude::*;

use crate::{
    api::session::member::get_members,
    core::session::SessionKey,
    frontend::session::{member::MembersList, SessionHeader, SessionNavBar},
    RootNavBar,
};

#[component]
pub fn SessionHome(session_key: ReadOnlySignal<String>) -> Element {
    let members =
        use_resource(move || async move { get_members(SessionKey(session_key.to_string())).await });

    rsx! {
        RootNavBar {}
        SessionHeader { session_key }
        SessionNavBar { session_key }
        if let Some(Ok(members)) = members.read().as_ref() {
            MembersList { members: members.clone() }
        }
    }
}
