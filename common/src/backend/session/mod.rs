use sqlx::{Pool, Sqlite};
use thiserror::Error;
use uuid::Uuid;

use crate::session::{Session, SessionDiscordId, SessionKey, SessionName};

use super::DatabaseError;

pub mod city;
pub mod member;
pub mod tech;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Database error: `{0}`")]
    Database(#[from] DatabaseError),
}

async fn session_exist(
    pool: &Pool<Sqlite>,
    discord_id: &SessionDiscordId,
) -> Result<bool, SessionError> {
    Ok(
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM session WHERE discord_id = ?1)")
            .bind(discord_id.0.clone())
            .fetch_one(pool)
            .await
            .map_err(DatabaseError::from)?,
    )
}

pub async fn ensure_session(
    pool: &Pool<Sqlite>,
    discord_id: &SessionDiscordId,
    name: &SessionName,
) -> Result<Session, SessionError> {
    if !session_exist(pool, discord_id).await? {
        let key = SessionKey(Uuid::new_v4().to_string());
        sqlx::query!(
            r#"INSERT INTO session ( key, discord_id, name ) VALUES ( ?1, ?2, ?3 )"#,
            key.0,
            discord_id.0,
            name.0,
        )
        .execute(pool)
        .await
        .map_err(DatabaseError::from)?;
    }

    let mut session = get_session(pool, discord_id).await?;
    if session.name().0 != name.0 {
        session.set_name(name.clone());
        sqlx::query!(r#"UPDATE session SET name = ?1"#, name.0,)
            .execute(pool)
            .await
            .map_err(DatabaseError::from)?;
    }

    Ok(session)
}

pub async fn get_session(
    pool: &Pool<Sqlite>,
    discord_id: &SessionDiscordId,
) -> Result<Session, SessionError> {
    let raw_session = sqlx::query!(
        r#"SELECT name, key FROM session WHERE discord_id = ?1"#,
        discord_id.0
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(Session::builder()
        .name(SessionName(raw_session.name))
        .key(SessionKey(raw_session.key))
        .build())
}
