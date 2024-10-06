use common::{
    backend::session::city::{add_city, city_exist},
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
    _ctx: Context<'_>,
    name: String,
    user: Option<serenity::User>,
) -> Result<(), Error> {
    Ok(())
}
