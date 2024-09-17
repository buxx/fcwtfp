use dioxus::prelude::*;

use crate::{
    api::session::member::{get_members, post_new_member},
    core::session::{
        member::{MemberName, Members},
        SessionKey,
    },
};

#[component]
pub fn AddMember(session_key: ReadOnlySignal<String>, members: Signal<Members>) -> Element {
    let mut name = use_signal(|| String::from(""));
    let mut added = use_signal(|| false);

    rsx! {
        if added() {
            p {
                color: "green",
                "{name} successfully added"
            }
        }
        form {
            onsubmit: move |_| async move {
                if !name.to_string().trim().is_empty() {
                    add_member(SessionKey(session_key.to_string()), members, MemberName(name.to_string().trim().to_string())).await;
                    added.set(true);
                }
            },
            label { r#for: "name", "Name" },
            input {
                value: "{name}",
                id: "name",
                oninput: move |evt| name.set(evt.value().clone()),
            }
            input { r#type: "submit", value: "Add" },
        }
    }
}

async fn add_member(key: SessionKey, mut members: Signal<Members>, name: MemberName) {
    post_new_member(key.clone(), name).await.unwrap();
    *members.write() = get_members(key.clone()).await.unwrap();
}
