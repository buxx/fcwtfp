use common::{
    backend::session::city::{add_city, city_exist, find_city_by_partial_name, remove_city},
    session::member::MemberDiscordId,
};
use poise::serenity_prelude::{self as serenity};

use crate::{command::extract_from_context, Context, Error};

use super::CommandError;

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
    if let Err(error) = _add(ctx, name, user).await {
        if let CommandError::Managed(_) = error {
            return Err(Box::new(error));
        }

        eprintln!("Error during add city command: {}", error);
        return Err(Box::new(error));
    }

    Ok(())
}

async fn _add(
    ctx: Context<'_>,
    name: String,
    user: Option<serenity::User>,
) -> Result<(), CommandError> {
    let (pool, author, session) = extract_from_context(ctx).await?;
    let user_id = user.clone().map(|u| u.id).unwrap_or(author.user.id);

    if city_exist(
        &pool,
        session.key(),
        &MemberDiscordId(user_id.to_string()),
        &name,
    )
    .await
    .map_err(CommandError::managed)?
    {
        ctx.say("This city is already known".to_string()).await?;
        return Ok(());
    }

    add_city(
        &pool,
        session.key(),
        &MemberDiscordId(user_id.to_string()),
        &name,
    )
    .await
    .map_err(CommandError::unexpected)?;

    ctx.say("City added".to_string()).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_city_name"] name: String,
    user: serenity::User,
) -> Result<(), Error> {
    let (pool, _, session) = extract_from_context(ctx).await?;
    let user_id = user.id;

    remove_city(
        &pool,
        session.key(),
        &MemberDiscordId(user_id.to_string()),
        &name,
    )
    .await
    .map_err(CommandError::unexpected)?;

    ctx.say("City removed".to_string()).await?;
    Ok(())
}

async fn autocomplete_city_name(
    ctx: Context<'_>,
    partial: &str,
) -> Vec<serenity::AutocompleteChoice> {
    let (pool, session) = match extract_from_context(ctx).await {
        Ok((pool, _, session)) => (pool, session),
        Err(error) => {
            eprintln!("Error during city remove autocomplete (extract from contexts): {error}");
            return vec![];
        }
    };

    match find_city_by_partial_name(&pool, session.key(), partial).await {
        Ok(cities) => {
            //
            cities
                .iter()
                .map(|city| {
                    serenity::AutocompleteChoice::new(
                        city.name().to_string(),
                        city.name().to_string(),
                    )
                })
                .collect()
        }
        Err(error) => {
            eprintln!("Error during city remove autocomplete (find city): {error}");
            vec![]
        }
    }
}
