use std::str::FromStr;

use common::{
    backend::{
        pool,
        session::{
            get_session,
            tech::{get_technologies_state, set_technology_state},
            SessionError,
        },
    },
    session::{
        member::MemberDiscordId,
        tech::{State, Technology, TechnologyStateError},
        SessionDiscordId,
    },
    DatabaseError,
};
use poise::serenity_prelude::{self as serenity};
use strum::IntoEnumIterator;
use thiserror::Error as ThisError;

use crate::{tech::TechnologyStateMarkdown, Context, Error};

#[derive(ThisError, Debug)]
pub enum TechCommandError {
    #[error("Incorrect given technology")]
    UnexpectedTechnologyInput,
    #[error("Internal error")]
    Database(#[from] DatabaseError),
    #[error("Internal error")]
    Session(#[from] SessionError),
    #[error("Internal error")]
    TechnologyState(#[from] TechnologyStateError),
}

#[poise::command(slash_command, prefix_command)]
pub async fn tech(
    ctx: Context<'_>,
    state: State,
    #[autocomplete = "autocomplete_technology"] tech: String,
    user: Option<serenity::User>,
) -> Result<(), Error> {
    let technology = match Technology::from_str(&tech) {
        Ok(technology) => technology,
        Err(_) => return Err(Box::new(TechCommandError::UnexpectedTechnologyInput)),
    };

    let pool = match pool().await {
        Ok(pool) => pool,
        Err(error) => {
            eprintln!("Error during `tech` command: {:?}", error);
            return Err(Box::new(TechCommandError::from(error)));
        }
    };

    if let Some(author) = ctx.author_member().await {
        if let Some(guild_id) = ctx.guild_id() {
            let session = match get_session(&pool, &SessionDiscordId(guild_id.to_string())).await {
                Ok(pool) => pool,
                Err(error) => {
                    eprintln!("Error during `tech` command: {:?}", error);
                    return Err(Box::new(TechCommandError::from(error)));
                }
            };

            let user_id = user.clone().map(|u| u.id).unwrap_or(author.user.id);
            if let Err(error) = set_technology_state(
                &pool,
                session.key(),
                &MemberDiscordId(user_id.to_string()),
                &technology,
                &state,
            )
            .await
            {
                eprintln!("Error during `tech` command: {:?}", error);
                return Err(Box::new(TechCommandError::from(error)));
            }

            let technology_state = match get_technologies_state(&pool, session.key()).await {
                Ok(technology_state) => technology_state,
                Err(error) => {
                    eprintln!("Error during `tech` command: {:?}", error);
                    return Err(Box::new(TechCommandError::from(error)));
                }
            };

            let md = TechnologyStateMarkdown::from(technology_state);
            ctx.say(md.0).await?;
        } else {
            eprintln!("Error during `tech` command: no guild id was in context");
        }
    } else {
        eprintln!("Error during `tech` command: no author");
    }

    Ok(())
}

async fn autocomplete_technology(
    _: Context<'_>,
    partial: &str,
) -> Vec<serenity::AutocompleteChoice> {
    Technology::iter()
        .filter(|technology| {
            technology
                .to_string()
                .to_ascii_lowercase()
                .contains(&partial.to_ascii_lowercase())
        })
        .map(|technology| {
            serenity::AutocompleteChoice::new(technology.to_string(), technology.to_string())
        })
        .collect()
}
