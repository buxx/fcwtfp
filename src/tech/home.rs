use dioxus::prelude::*;

use crate::{
    session::{SessionHeader, SessionNavBar},
    RootNavBar,
};

#[component]
pub fn TechHome(session_key: ReadOnlySignal<String>) -> Element {
    rsx! {
        RootNavBar {}
        SessionHeader { session_key }
        SessionNavBar { session_key }
        h2 { "Technologies" }
    }
}
