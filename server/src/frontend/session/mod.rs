use common::session::SessionKey;
use create::Create;
use dioxus::prelude::*;
use join::Join;

use super::router::Route;
use crate::{api::session::get_session, RootNavBar};

pub mod create;
pub mod home;
pub mod join;
pub mod member;
pub mod tech;

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
