use dioxus::prelude::*;

use super::create::Create;
use super::join::Join;
use crate::NavBar;

#[component]
pub fn SessionHome() -> Element {
    rsx! {
        NavBar { }
        Create {}
        Join {}
    }
}
