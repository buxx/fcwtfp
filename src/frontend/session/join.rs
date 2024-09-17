use dioxus::prelude::*;

use crate::frontend::router::Route;

#[component]
pub fn Join() -> Element {
    let mut key = use_signal(|| String::from(""));

    rsx! {
        h2 { "Join session" }
        form {
            onsubmit: move |_| join_session(key.to_string()),
            label { r#for: "key", "Key" },
            input {
                value: "{key}",
                id: "key",
                oninput: move |evt| key.set(evt.value().clone()),
            }
            input { r#type: "submit", value: "Join" },
        }
    }
}

fn join_session(session_key: String) {
    navigator().replace(Route::SessionHome { session_key });
}
