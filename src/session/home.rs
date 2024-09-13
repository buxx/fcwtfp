use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::storage::{self};
use crate::{
    session::{SessionHeader, SessionKey, SessionNavBar},
    RootNavBar,
};

use super::member::Members;

#[component]
pub fn SessionHome(session_key: ReadOnlySignal<String>) -> Element {
    let session_members = use_server_future(move || async move {
        get_session_members(SessionKey(session_key.to_string()))
            .await
            .unwrap()
    })?;

    rsx! {
        RootNavBar {}
        SessionHeader { session_key }
        SessionNavBar { session_key }
        p {
            "Members: {session_members()?}"
        }
        // TODO: add/remove member
    }
}

#[server(GetSessionMembers)]
async fn get_session_members(session_key: SessionKey) -> Result<Members, ServerFnError> {
    Ok(storage::session::member::get_session_members(&session_key).unwrap())
}
