#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use frontend::router::Route;

mod api;
#[cfg(feature = "server")]
mod backend;
mod core;
mod frontend;

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
pub fn RootNavBar() -> Element {
    rsx! {
        ul {
            li {
                Link { to: Route::Root {}, "Home" }
            }
            li {
                Link { to: Route::SessionRoot {}, "Session" }
            }
        }
    }
}

#[component]
pub fn Root() -> Element {
    rsx! {
        RootNavBar { }
    }
}
