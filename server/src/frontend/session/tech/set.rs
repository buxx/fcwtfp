use common::session::tech::Technology;
use dioxus::prelude::*;
use std::str::FromStr;
use std::string::ToString;
use strum::IntoEnumIterator;

use crate::api::session::member::get_members;
use crate::api::session::tech::{get_technologies_state, set_technology_state};
use crate::frontend::generic::message::SimpleMessage;
use common::session::member::MemberName;
use common::session::tech::{State, TechnologyState};
use common::session::SessionKey;

#[component]
pub fn SetTechnologyState(
    session_key: ReadOnlySignal<String>,
    state: Signal<TechnologyState>,
) -> Element {
    let members_ = use_server_future(move || async move {
        get_members(SessionKey(session_key.to_string()))
            .await
            .unwrap()
    })?;

    if let Some(members) = members_() {
        if !members.is_empty() {
            let mut selected_member_name =
                use_signal(|| members.0.first().expect("Tested before").name().0.clone());
            let mut selected_technology =
                use_signal(|| Technology::iter().next().expect("There is always tech"));
            let mut selected_state = use_signal(|| State::Researching);

            rsx! {
                h3 { "Set member technology state" }
                form {
                    onsubmit: move |_| {
                        async move {
                            apply_technology_state(
                                SessionKey(session_key.to_string()),
                                MemberName(selected_member_name.to_string()),
                                selected_technology(),
                                selected_state(),
                                state
                            ).await
                        }
                    },
                    select {
                        id: "member_name",
                        oninput: move |evt| selected_member_name.set(evt.value()),
                        for member in members.0 {
                            option {
                                value: member.name().0.clone(),
                                "{member.name().0.clone()}"
                            }
                        }

                    }
                    select {
                        id: "technology_name",
                        oninput: move |evt| selected_technology.set(Technology::from_str(&evt.value()).unwrap()),
                        for technology in Technology::iter() {
                            option {
                                value: technology.to_string(),
                                "{technology.to_string()}"
                            }
                        }

                    }
                    select {
                        id: "state_name",
                        oninput: move |evt| selected_state.set(State::from_str(&evt.value()).unwrap()),
                        for action in State::iter() {
                            option {
                                value: action.to_string(),
                                "{action.to_string()}"
                            }
                        }

                    }
                    input { r#type: "submit", value: "Apply" }

                }
            }
        } else {
            rsx! {
                SimpleMessage {
                    text: "No member yet"
                }
            }
        }
    } else {
        rsx! {}
    }
}

async fn apply_technology_state(
    session_key: SessionKey,
    member_name: MemberName,
    technology: Technology,
    action: State,
    mut state: Signal<TechnologyState>,
) {
    set_technology_state(session_key.clone(), member_name, technology, action)
        .await
        .unwrap();
    let new_state = get_technologies_state(session_key).await.unwrap();
    state.set(new_state);
}
