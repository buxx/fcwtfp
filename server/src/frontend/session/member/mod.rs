use dioxus::prelude::*;

use dioxus::{prelude::Element, signals::ReadOnlySignal};

use crate::frontend::generic::loading::Loading;
use crate::frontend::generic::Res;
use common::session::member::Members;

pub mod add;
pub mod home;
pub mod remove;

#[component]
pub fn MembersList(members: ReadOnlySignal<Res<Members>>) -> Element {
    rsx! {
        if let Res::Loaded(members) = members() {
            p {
                "Members: {members}"
            }
        } else {
            Loading {}
        }
    }
}
