use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::session::SessionName;
use crate::Route;

#[cfg(feature = "server")]
use crate::storage::{self};

use super::Session;

#[component]
pub fn Create() -> Element {
    let mut name = use_signal(|| String::from(""));

    rsx! {
        h2 { "Create session" }
        form {
            onsubmit: move |_| async move {
                let session = create_and_join_session(name.to_string()).await.unwrap();
                navigator().replace(Route::SessionHome {
                    session_key: session.key().0.clone(),
                });
            },
            label { r#for: "name", "Name" },
            input {
                value: "{name}",
                id: "name",
                oninput: move |evt| name.set(evt.value().clone()),
            }
            input { r#type: "submit", value: "Create" },
        }
    }
}

#[server(CreateSession)]
async fn create_and_join_session(name: String) -> Result<Session, ServerFnError> {
    let session = storage::session::create_session(SessionName(name))?;
    Ok(session)
}
