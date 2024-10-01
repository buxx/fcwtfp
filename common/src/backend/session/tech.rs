use sqlx::{Pool, Sqlite};
use std::{collections::HashMap, str::FromStr};

use strum::IntoEnumIterator;

use crate::{
    backend::DatabaseError,
    session::{
        member::MemberDiscordId,
        tech::{State, TechnologiesState, Technology, TechnologyInState, TechnologyStateError},
        SessionKey,
    },
};

use super::member::get_members;

pub async fn get_technologies_state(
    pool: &Pool<Sqlite>,
    key: &SessionKey,
) -> Result<TechnologiesState, TechnologyStateError> {
    let mut state = TechnologiesState::default();
    let members = get_members(pool, key).await?;
    let mut search = vec![];
    let mut done = vec![];
    let mut search_members = HashMap::new();
    let mut done_members = HashMap::new();

    for member in members.0 {
        let member_searching = get_member_searching(pool, key, member.discord_id()).await?;
        for technology in member_searching.clone() {
            search_members
                .entry(technology)
                .or_insert(vec![])
                .push(member.clone());
        }

        let member_done = get_member_done(pool, key, member.discord_id()).await?;
        for technology in member_done.clone() {
            done_members
                .entry(technology)
                .or_insert(vec![])
                .push(member.clone());
        }

        search.extend(member_searching);
        done.extend(member_done);
    }

    for technology in Technology::iter() {
        if search.contains(&technology) {
            state.add_search(TechnologyInState::new(
                technology.clone(),
                search_members.get(&technology).unwrap_or(&vec![]).to_vec(),
            ));
        }

        if done.contains(&technology) {
            state.add_done(TechnologyInState::new(
                technology.clone(),
                done_members.get(&technology).unwrap_or(&vec![]).to_vec(),
            ));
        }
    }

    Ok(state)
}

// TODO: refactor with get_member_done
pub async fn get_member_searching(
    pool: &Pool<Sqlite>,
    key: &SessionKey,
    member_discord_id: &MemberDiscordId,
) -> Result<Vec<Technology>, TechnologyStateError> {
    let mut technologies = vec![];

    for raw_technology in sqlx::query!(
        r#"
            SELECT name FROM session_tech
            WHERE session_key = ?1 AND session_member_discord_id = ?2 AND done = 0
        "#,
        key.0,
        member_discord_id.0,
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)?
    {
        technologies.push(Technology::from_str(&raw_technology.name).unwrap());
    }

    Ok(technologies)
}

// TODO: refactor with get_member_searching
pub async fn get_member_done(
    pool: &Pool<Sqlite>,
    key: &SessionKey,
    member_discord_id: &MemberDiscordId,
) -> Result<Vec<Technology>, TechnologyStateError> {
    let mut technologies = vec![];

    for raw_technology in sqlx::query!(
        r#"
            SELECT name FROM session_tech
            WHERE session_key = ?1 AND session_member_discord_id = ?2 AND done = 1
        "#,
        key.0,
        member_discord_id.0,
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)?
    {
        technologies.push(Technology::from_str(&raw_technology.name).unwrap());
    }

    Ok(technologies)
}

pub async fn set_technology_state(
    pool: &Pool<Sqlite>,
    key: &SessionKey,
    member_discord_id: &MemberDiscordId,
    technology: &Technology,
    state: &State,
) -> Result<(), TechnologyStateError> {
    let technology_name = technology.to_string();
    match state {
        State::Researching => sqlx::query!(
            r#"
                    REPLACE INTO session_tech (session_key, session_member_discord_id, name, done)
                    VALUES (?1, ?2, ?3, ?4)
            "#,
            key.0,
            member_discord_id.0,
            technology_name,
            "0"
        ),
        State::Done => sqlx::query!(
            r#"
                    REPLACE INTO session_tech (session_key, session_member_discord_id, name, done)
                    VALUES (?1, ?2, ?3, ?4)
            "#,
            key.0,
            member_discord_id.0,
            technology_name,
            "1"
        ),
        State::Cancel => sqlx::query!(
            r#"
                    DELETE FROM session_tech
                    WHERE session_key = ?1 AND session_member_discord_id = ?2 AND name = ?3
            "#,
            key.0,
            member_discord_id.0,
            technology_name,
        ),
    }
    .execute(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(())
}
