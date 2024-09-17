use dioxus::prelude::*;

use crate::core::session::tech::TechnologyState;

#[component]
pub fn TechnologyStateDisplay(state: Signal<TechnologyState>) -> Element {
    let state_ = state();

    rsx! {
        h3 { "Done by at least one member" }
        if !state_.done().is_empty() {
            ul {
                for technology in state_.done() {
                    li {
                        "{technology.to_string()}"
                    }
                }
            }
        } else {
            p { "No one" }
        }
        h3 { "Searched by at least one member" }
        if !state_.search().is_empty() {
            ul {
                for technology in state_.search() {
                    li {
                        "{technology.to_string()}"
                    }
                }
            }
        } else {
            p { "No one" }
        }
    }
}
