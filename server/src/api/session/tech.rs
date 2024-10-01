use dioxus::prelude::*;

#[cfg(feature = "server")]
use common::backend::{self};

use common::session::{
    member::MemberName,
    tech::{State, TechnologiesState, Technology},
    SessionKey,
};

#[server(GetTechnologiesState)]
pub async fn get_technologies_state(
    session_key: SessionKey,
) -> Result<TechnologiesState, ServerFnError> {
    Ok(backend::session::tech::get_technologies_state(&session_key)
        .await
        .unwrap())
}

#[server(SetTechnologyState)]
pub async fn set_technology_state(
    session_key: SessionKey,
    member_name: MemberName,
    technology: Technology,
    state: State,
) -> Result<(), ServerFnError> {
    Ok(backend::session::tech::set_technology_state(
        &session_key,
        &member_name,
        &technology,
        &state,
    )
    .await
    .unwrap())
}
