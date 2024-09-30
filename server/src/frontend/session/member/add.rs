use dioxus::prelude::*;

use crate::{
    api::session::member::{get_members, post_new_member},
    frontend::generic::{message::SimpleMessage, Res},
};
use common::session::{
    member::{MemberName, Members},
    SessionKey,
};

#[component]
pub fn AddMember(session_key: ReadOnlySignal<String>, members: Signal<Res<Members>>) -> Element {
    let mut error = use_signal(|| Some(String::from("")));
    let mut name = use_signal(|| String::from(""));
    let mut added = use_signal(|| false);

    rsx! {
        if let Some(error_) = error() {
            SimpleMessage {
                color: "red",
                text: error_
            }
        }
        if added() {
            p {
                color: "green",
                "{name} successfully added"
            }
        }
        form {
            onsubmit: move |_| async move {
                if !name.to_string().trim().is_empty() {
                    members.set(Res::Loading);
                    if let Err(add_error) = add_member(
                        SessionKey(session_key.to_string()),
                        members,
                        MemberName(name.to_string().trim().to_string())
                    ).await {
                        error.set(Some(add_error.to_string()))
                    };
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

async fn add_member(
    key: SessionKey,
    mut members: Signal<Res<Members>>,
    name: MemberName,
) -> Result<(), ServerFnError> {
    post_new_member(key.clone(), name).await?;
    *members.write() = Res::Loaded(get_members(key.clone()).await?);
    Ok(())
}
