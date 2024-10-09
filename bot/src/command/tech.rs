use std::str::FromStr;

use common::{
    backend::session::tech::{get_technologies_state, set_technology_state},
    session::{
        member::MemberDiscordId,
        tech::{State, Technology},
    },
};
use poise::serenity_prelude::{self as serenity};
use strum::IntoEnumIterator;

use crate::{command::CommandError, tech::TechnologyStateMarkdown, Context, Error};

use super::extract_from_context;

#[poise::command(slash_command, prefix_command, subcommands("set", "list"))]
pub async fn tech(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn set(
    ctx: Context<'_>,
    state: State,
    #[autocomplete = "autocomplete_technology"] tech: String,
    user: Option<serenity::User>,
) -> Result<(), Error> {
    let technology = Technology::from_str(&tech)
        .map_err(|_| Box::new(CommandError::Managed("Unknown technology".to_string())))?;

    if let Err(error) = _tech(ctx, state, technology, user).await {
        if let CommandError::Managed(_) = error {
            return Err(Box::new(error));
        }

        eprintln!("Error during set tech command: {}", error);
        return Err(Box::new(error));
    }

    Ok(())
}

pub async fn _tech(
    ctx: Context<'_>,
    state: State,
    technology: Technology,
    user: Option<serenity::User>,
) -> Result<(), CommandError> {
    let (pool, author, session) = extract_from_context(ctx).await?;
    let user_id = user.clone().map(|u| u.id).unwrap_or(author.user.id);

    set_technology_state(
        &pool,
        session.key(),
        &MemberDiscordId(user_id.to_string()),
        &technology,
        &state,
    )
    .await
    .map_err(CommandError::unexpected)?;

    let technology_state = get_technologies_state(&pool, session.key())
        .await
        .map_err(CommandError::unexpected)?;
    let md = TechnologyStateMarkdown::from(technology_state);
    ctx.say(md.0).await?;

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

#[poise::command(slash_command, prefix_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let (pool, _, session) = extract_from_context(ctx).await?;

    let technology_state = get_technologies_state(&pool, session.key())
        .await
        .map_err(CommandError::unexpected)?;
    let md = TechnologyStateMarkdown::from(technology_state);
    ctx.say(md.0).await?;

    Ok(())
}
