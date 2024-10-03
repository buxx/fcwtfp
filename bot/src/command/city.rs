use common::{
    backend::{
        pool,
        session::{
            city::{add_city, city_exist},
            get_session, SessionError,
        },
    },
    session::{
        city::CityError, member::MemberDiscordId, tech::TechnologyStateError, SessionDiscordId,
    },
    DatabaseError,
};
use poise::serenity_prelude::{self as serenity};
use thiserror::Error as ThisError;

use crate::{Context, Error};

#[derive(ThisError, Debug)]
pub enum CityCommandError {
    #[error("Internal error")]
    Database(#[from] DatabaseError),
    #[error("Internal error")]
    Session(#[from] SessionError),
    #[error("Internal error")]
    City(#[from] CityError),
    #[error("Internal error")]
    TechnologyState(#[from] TechnologyStateError),
}

#[poise::command(slash_command, prefix_command, subcommands("add", "remove"))]
pub async fn city(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    name: String,
    user: Option<serenity::User>,
) -> Result<(), Error> {
    let pool = match pool().await {
        Ok(pool) => pool,
        Err(error) => {
            eprintln!("Error during `tech` command: {:?}", error);
            return Err(Box::new(CityCommandError::from(error)));
        }
    };

    if let Some(author) = ctx.author_member().await {
        if let Some(guild_id) = ctx.guild_id() {
            let session = match get_session(&pool, &SessionDiscordId(guild_id.to_string())).await {
                Ok(pool) => pool,
                Err(error) => {
                    eprintln!("Error during `tech` command: {:?}", error);
                    return Err(Box::new(CityCommandError::from(error)));
                }
            };

            let user_id = user.clone().map(|u| u.id).unwrap_or(author.user.id);
            match city_exist(
                &pool,
                session.key(),
                &MemberDiscordId(user_id.to_string()),
                &name,
            )
            .await
            {
                Ok(true) => {
                    ctx.say("This city is already known".to_string()).await?;
                    return Ok(());
                }
                Ok(false) => {}
                Err(error) => {
                    eprintln!("Error during `tech` command: {:?}", error);
                    return Err(Box::new(CityCommandError::from(error)));
                }
            }

            if let Err(error) = add_city(
                &pool,
                session.key(),
                &MemberDiscordId(user_id.to_string()),
                &name,
            )
            .await
            {
                eprintln!("Error during `tech` command: {:?}", error);
                return Err(Box::new(CityCommandError::from(error)));
            }

            ctx.say("City added".to_string()).await?;
            return Ok(());
        } else {
            eprintln!("Error during `tech` command: no guild id was in context");
        }
    } else {
        eprintln!("Error during `tech` command: no author");
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    _ctx: Context<'_>,
    name: String,
    user: Option<serenity::User>,
) -> Result<(), Error> {
    Ok(())
}
