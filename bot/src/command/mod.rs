use std::{borrow::Cow, fmt::Display};

use common::{
    backend::{
        pool,
        session::{get_session, SessionError},
    },
    session::{Session, SessionDiscordId},
    DatabaseError,
};
use poise::serenity_prelude::{self as serenity};
use sqlx::{Pool, Sqlite};
use thiserror::Error as ThisError;

use crate::Context;

pub mod city;
pub mod tech;

#[derive(ThisError, Debug)]
pub enum CommandError {
    #[error("Internal error")]
    ExtractFromContext(#[from] ExtractFromContextError),
    #[error("{0}")]
    Managed(String),
    #[error("Internal error")]
    Unexpected(String),
    #[error("Internal error")]
    UnexpectedSerenity(#[from] serenity::Error),
}

impl CommandError {
    fn managed(error: impl Display) -> Self {
        Self::Managed(error.to_string())
    }

    fn unexpected(error: impl Display) -> Self {
        Self::Unexpected(error.to_string())
    }
}

#[derive(ThisError, Debug)]
pub enum ExtractFromContextError {
    #[error("Database error: {0}")]
    Database(DatabaseError),
    #[error("Session error: {0}")]
    Session(SessionError),
    #[error("No author id was in context")]
    NoAuthor,
    #[error("No guild id was in context")]
    NoGuild,
}

pub async fn extract_from_context(
    ctx: Context<'_>,
) -> Result<(Pool<Sqlite>, Cow<serenity::Member>, Session), ExtractFromContextError> {
    let pool = match pool().await {
        Ok(pool) => pool,
        Err(error) => {
            return Err(ExtractFromContextError::Database(error));
        }
    };

    if let Some(author) = ctx.author_member().await {
        if let Some(guild_id) = ctx.guild_id() {
            let session = match get_session(&pool, &SessionDiscordId(guild_id.to_string())).await {
                Ok(pool) => pool,
                Err(error) => {
                    return Err(ExtractFromContextError::Session(error));
                }
            };

            Ok((pool, author, session))
        } else {
            Err(ExtractFromContextError::NoGuild)
        }
    } else {
        Err(ExtractFromContextError::NoAuthor)
    }
}
