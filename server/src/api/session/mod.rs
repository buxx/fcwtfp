pub mod tech;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use common::backend::{self};
#[cfg(feature = "server")]
use common::session::SessionName;

use common::session::{Session, SessionKey};

pub mod member;

#[server(GetSession)]
pub async fn get_session(session_key: SessionKey) -> Result<Session, ServerFnError> {
    Ok(backend::session::get_session(&session_key).await.unwrap())
}

#[server(CreateSession)]
pub async fn create_and_join_session(name: String) -> Result<Session, ServerFnError> {
    Ok(backend::session::ensure_session(SessionName(name)).await?)
}
