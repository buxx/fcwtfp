use dioxus::prelude::*;

use crate::frontend::session::home::SessionHome;
use crate::frontend::session::member::home::MembersHome;
use crate::frontend::session::tech::home::TechHome;
use crate::frontend::session::SessionRoot;
use crate::Root;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum Route {
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
    #[end_nest]

    #[route("/")]
    Root {},
}
