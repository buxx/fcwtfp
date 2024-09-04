use dioxus::prelude::*;

use crate::post_server_data;

#[component]
pub fn Join() -> Element {
    let mut key = use_signal(|| String::from("..."));

    rsx! {
        h2 { "Join session" }
        form {
            onsubmit: move |_| async move {
                post_server_data(key.to_string()).await.unwrap();
            },
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
