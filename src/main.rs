#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use server_fn::codec::Json;

mod session;
#[cfg(feature = "server")]
mod storage;
mod tech;

use session::home::SessionHome;
use session::member::MembersHome;
use session::SessionRoot;
use tech::home::TechHome;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Root {},

    #[route("/session")]
    SessionRoot {},
    #[nest("/session")]
        #[nest("/:session_key")]
            #[route("/")]
            SessionHome {
                session_key: String,
            },
            #[route("/members")]
            MembersHome {
                session_key: String,
            },
            #[route("/tech")]
            TechHome { 
                session_key: String,
             },
        #[end_nest]

    #[route("/old")]
    Old {},
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn RootNavBar() -> Element {
    rsx! {
        ul {
            li {
                Link { to: Route::Root {}, "Home" }
            }
            li {
                Link { to: Route::SessionRoot {}, "Session management" }
            }
            li {
                Link { to: Route::Old {}, "Old" }
            }
        }
    }
}

#[component]
fn Old() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        RootNavBar { }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data :) : {text}"}
        }
    }
}

#[component]
fn Root() -> Element {
    rsx! {
        RootNavBar { }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(
  prefix = "/my_api",
  endpoint = "my_fn",
  output = Json
)]
pub async fn my_wacky_server_fn() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
