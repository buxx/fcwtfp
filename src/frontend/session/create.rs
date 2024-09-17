use dioxus::prelude::*;

use crate::api::session::create_and_join_session;
use crate::Route;

#[component]
pub fn Create() -> Element {
    let mut name: Signal<String> = use_signal(|| String::from(""));

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
