use dioxus::prelude::*;

use crate::{
    api::session::member::{delete_member, get_members},
    core::session::{
        member::{MemberName, Members},
        SessionKey,
    },
    frontend::generic::message::SimpleMessage,
};

#[component]
pub fn RemoveMember(session_key: ReadOnlySignal<String>, members: Signal<Members>) -> Element {
    let mut selected_name: Signal<Option<String>> = use_signal(|| None);
    use_memo(move || selected_name.set(default_name(&members.read().clone())));
    let mut removed: Signal<Option<String>> = use_signal(|| None);
    let members_ = members.read().clone();

    if let Some(remove_name) = selected_name() {
        let remove_name_ = remove_name.clone();

        rsx! {
            if let Some(removed_name) = removed() {
                SimpleMessage {
                    color: "green",
                    text: "{removed_name} successfully removed (and all its technologies and cities)."
                }
            } else {
                SimpleMessage {
                    text: "Remove an member will remove him and all its technologies and cities."
                }
            }

            form {
                onsubmit: move |_| {
                    let remove_name_ = remove_name.clone();
                    async move {
                        remove_member(SessionKey(session_key.to_string()), members, MemberName(remove_name_.clone())).await;
                        removed.set(Some(remove_name_));

                    }
                },
                label { r#for: "name", "Name" },
                select {
                    id: "name",
                    oninput: move |evt| selected_name.set(Some(evt.value().clone())),
                    for member in members_.0 {
                        option {
                            value: member.name().0.to_string(),
                            "{member.name().0.to_string()}"
                        }
                    }

                }
                input { r#type: "submit", value: "Remove {remove_name_}" }

            }

        }
    } else {
        rsx! {
            p {
                "No member yet"
            }
        }
    }
}

fn default_name(members: &Members) -> Option<String> {
    members.members().first().map(|m| m.name().0.to_string())
}

async fn remove_member(key: SessionKey, mut members: Signal<Members>, name: MemberName) -> Members {
    delete_member(key.clone(), name.clone()).await.unwrap();
    let new_members = get_members(key.clone()).await.unwrap();
    members.set(new_members.clone());
    new_members
}
