use common::session::{tech::TechnologyState, SessionKey};
use dioxus::prelude::*;

use crate::{
    api::session::tech::get_technologies_state,
    frontend::session::{
        tech::{set::SetTechnologyState, state::TechnologyStateDisplay},
        SessionHeader, SessionNavBar,
    },
    RootNavBar,
};

#[component]
pub fn TechHome(session_key: ReadOnlySignal<String>) -> Element {
    let members_ = use_server_future(move || async move {
        get_technologies_state(SessionKey(session_key.to_string()))
            .await
            .unwrap()
    })?;
    let state = use_signal(|| members_().unwrap_or(TechnologyState::default()));

    rsx! {
        RootNavBar {}
        SessionHeader { session_key }
        SessionNavBar { session_key }
        h2 { "Technologies" }
        TechnologyStateDisplay { state }
        SetTechnologyState { session_key, state }
    }
}
