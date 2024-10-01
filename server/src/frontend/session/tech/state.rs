use dioxus::prelude::*;

use common::session::tech::TechnologiesState;

#[component]
pub fn TechnologyStateDisplay(state: Signal<TechnologiesState>) -> Element {
    let state_ = state();

    rsx! {
        h3 { "Done by at least one member" }
        if !state_.done().is_empty() {
            ul {
                for technology in state_.done() {
                    li {
                        "{technology.technology_name()}"
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
                        "{technology.technology_name()}"
                    }
                }
            }
        } else {
            p { "No one" }
        }
    }
}
