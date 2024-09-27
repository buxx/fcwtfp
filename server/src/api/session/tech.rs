use common::tech::Technology;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::backend::{self};

use crate::core::session::{
    member::MemberName,
    tech::{State, TechnologyState},
    SessionKey,
};

#[server(GetTechnologiesState)]
pub async fn get_technologies_state(
    session_key: SessionKey,
) -> Result<TechnologyState, ServerFnError> {
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
