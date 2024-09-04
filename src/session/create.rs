use dioxus::prelude::*;

use dioxus_logger::tracing;

use crate::storage::Backend;

#[component]
pub fn Create() -> Element {
    let mut name = use_signal(|| String::from("..."));

    rsx! {
        h2 { "Create session" }
        form {
            onsubmit: move |_| async move {
                let key = create_session(name.to_string()).await.unwrap();
                tracing::info!("Created session: {}", key);
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

#[server(CrateSession)]
async fn create_session(name: String) -> Result<String, ServerFnError> {
    let storage: Backend = extract().await?;
    tracing::info!("Create session: {}", name);
    Ok(format!("key{}", storage.foo))
}
