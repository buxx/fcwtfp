use bon::builder;
use create::Create;
use dioxus::prelude::*;
use join::Join;
use serde::{Deserialize, Serialize};

use crate::{RootNavBar, Route};

pub mod create;
pub mod home;
pub mod join;
pub mod member;

#[cfg(feature = "server")]
use crate::storage::{self};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionName(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionKey(pub String);

#[builder]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    name: SessionName,
    key: SessionKey,
}

impl Session {
    pub fn name(&self) -> &SessionName {
        &self.name
    }

    pub fn key(&self) -> &SessionKey {
        &self.key
    }
}

#[component]
pub fn SessionNavBar(session_key: ReadOnlySignal<String>) -> Element {
    rsx! {
        ul {
            li {
                Link {
                    to: Route::SessionHome { session_key: session_key.to_string() },
                    "Home"
                }
            }
            li {
                Link {
                    to: Route::MembersHome { session_key: session_key.to_string() },
                    "Members"
                }
            }
            li {
                Link {
                    to: Route::TechHome { session_key: session_key.to_string() },
                    "Technologies"
                }
            }
        }
    }
}

#[component]
pub fn SessionRoot() -> Element {
    rsx! {
        RootNavBar {}
        Create {}
        Join {}
    }
}

#[component]
pub fn SessionHeader(session_key: ReadOnlySignal<String>) -> Element {
    let session = use_server_future(move || async move {
        get_session(SessionKey(session_key.to_string()))
            .await
            .unwrap()
    })?;

    rsx! {
        h1 { "Session {session()?.name().0}" }
    }
}

#[server(GetSession)]
async fn get_session(session_key: SessionKey) -> Result<Session, ServerFnError> {
    Ok(storage::session::get_session(&session_key).unwrap())
}
