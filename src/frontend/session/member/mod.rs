use dioxus::prelude::*;

use dioxus::{prelude::Element, signals::ReadOnlySignal};

use crate::core::session::member::Members;

pub mod add;
pub mod home;
pub mod remove;

#[component]
pub fn MembersList(members: ReadOnlySignal<Members>) -> Element {
    rsx! {
        p {
            "Members: {members}"
        }
    }
}
