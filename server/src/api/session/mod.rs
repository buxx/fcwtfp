pub mod tech;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::backend::{self};
#[cfg(feature = "server")]
use crate::core::session::SessionName;

use crate::core::session::{Session, SessionKey};

pub mod member;

#[server(GetSession)]
pub async fn get_session(session_key: SessionKey) -> Result<Session, ServerFnError> {
    Ok(backend::session::get_session(&session_key).await.unwrap())
}

#[server(CreateSession)]
pub async fn create_and_join_session(name: String) -> Result<Session, ServerFnError> {
    Ok(backend::session::create_session(SessionName(name)).await?)
}
