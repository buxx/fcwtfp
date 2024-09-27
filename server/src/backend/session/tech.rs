use std::str::FromStr;

use common::tech::Technology;
use strum::IntoEnumIterator;

use crate::{
    backend::{connection, DatabaseError},
    core::session::{
        member::MemberName,
        tech::{State, TechnologyState, TechnologyStateError},
        SessionKey,
    },
};

use super::member::get_members;

pub async fn get_technologies_state(
    key: &SessionKey,
) -> Result<TechnologyState, TechnologyStateError> {
    let mut state = TechnologyState::default();
    let members = get_members(key).await?;
    let mut search = vec![];
    let mut done = vec![];

    for member in members.0 {
        let member_searching = get_member_searching(key, &member.name()).await?;
        let member_done = get_member_done(key, &member.name()).await?;
        search.extend(member_searching);
        done.extend(member_done);
    }

    for technology in Technology::iter() {
        if search.contains(&technology) {
            state.add_search(technology.clone());
        }

        if done.contains(&technology) {
            state.add_done(technology.clone());
        }
    }

    Ok(state)
}

// TODO: refactor with get_member_done
pub async fn get_member_searching(
    key: &SessionKey,
    member_name: &MemberName,
) -> Result<Vec<Technology>, TechnologyStateError> {
    let mut technologies = vec![];

    for raw_technology in sqlx::query!(
        r#"
            SELECT name FROM session_tech
            WHERE session_key = ?1 AND session_member_name = ?2 AND done = 0
        "#,
        key.0,
        member_name.0,
    )
    .fetch_all(&mut *connection().await?)
    .await
    .map_err(DatabaseError::from)?
    {
        technologies.push(Technology::from_str(&raw_technology.name).unwrap());
    }

    Ok(technologies)
}

// TODO: refactor with get_member_searching
pub async fn get_member_done(
    key: &SessionKey,
    member_name: &MemberName,
) -> Result<Vec<Technology>, TechnologyStateError> {
    let mut technologies = vec![];

    for raw_technology in sqlx::query!(
        r#"
            SELECT name FROM session_tech
            WHERE session_key = ?1 AND session_member_name = ?2 AND done = 1
        "#,
        key.0,
        member_name.0,
    )
    .fetch_all(&mut *connection().await?)
    .await
    .map_err(DatabaseError::from)?
    {
        technologies.push(Technology::from_str(&raw_technology.name).unwrap());
    }

    Ok(technologies)
}

pub async fn set_technology_state(
    key: &SessionKey,
    member_name: &MemberName,
    technology: &Technology,
    state: &State,
) -> Result<(), TechnologyStateError> {
    let technology_name = technology.to_string();
    match state {
        State::Researching => sqlx::query!(
            r#"
                    REPLACE into session_tech (session_key, session_member_name, name, done)
                    VALUES (?1, ?2, ?3, ?4)
            "#,
            key.0,
            member_name.0,
            technology_name,
            "0"
        ),
        State::Done => sqlx::query!(
            r#"
                    REPLACE into session_tech (session_key, session_member_name, name, done)
                    VALUES (?1, ?2, ?3, ?4)
            "#,
            key.0,
            member_name.0,
            technology_name,
            "1"
        ),
        State::Cancel => sqlx::query!(
            r#"
                    DELETE FROM session_tech
                    WHERE session_key = ?1 AND session_member_name = ?2 AND name = ?3
            "#,
            key.0,
            member_name.0,
            technology_name,
        ),
    }
    .execute(&mut *connection().await?)
    .await
    .map_err(DatabaseError::from)?;

    Ok(())
}
