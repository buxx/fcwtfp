use poise::serenity_prelude::{self as serenity, futures::StreamExt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Error during guild synchronisation: {0:?}")]
    SyncGuilds(Vec<(serenity::GuildId, SyncGuildError)>),
    #[error("Error during members synchronisation: {0:?}")]
    SyncMembers(Vec<(serenity::GuildId, Option<serenity::UserId>, SyncMemberError)>),
}

#[derive(Error, Debug)]
pub enum SyncMemberError {
    #[error("Serenity: {0}")]
    SerenityError(#[from] serenity::Error),
}

#[derive(Error, Debug)]
pub enum SyncGuildError {
    #[error("Serenity: {0}")]
    SerenityError(#[from] serenity::Error),
}

pub async fn sync(ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), SyncError> {
    // let mut db = connection();

    sync_guilds(ctx, ready).await?;
    sync_members(ctx, ready).await?;

    Ok(())
}

async fn sync_guilds(ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), SyncError> {
    let mut errors: Vec<(serenity::GuildId, Option<serenity::UserId>, SyncMemberError)> = vec![];

    // for guild in &ready.guilds {
    //     if let Err(error) = sync_guild(&guild.id).await {
    //         errors.push((guild.id, error));
    //     }
    // }

    if !errors.is_empty() {
        return Err(SyncError::SyncMembers(errors));
    }

    Ok(())
}

async fn sync_members(ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), SyncError> {
    let mut errors: Vec<(serenity::GuildId, Option<serenity::UserId>, SyncMemberError)> = vec![];

    for guild in &ready.guilds {
        let mut members = serenity::MembersIter::<serenity::Http>::stream(&ctx, guild.id).boxed();
        while let Some(member_result) = members.next().await {
            match member_result {
                Ok(member) => {
                    if let Err(error) = sync_member(&guild.id, &member).await {
                        errors.push((guild.id, Some(member.user.id), error));
                    }
                }
                Err(error) => {
                    errors.push((guild.id, None, SyncMemberError::from(error)));
                }
            }
        }
    }

    if !errors.is_empty() {
        return Err(SyncError::SyncMembers(errors));
    }

    Ok(())
}

async fn sync_guild(guild_id: &serenity::GuildId) -> Result<(), SyncMemberError> {
    Ok(())
}

async fn sync_member(
    guild_id: &serenity::GuildId,
    member: &serenity::Member,
) -> Result<(), SyncMemberError> {
    println!(
        "{} is {} ({})",
        member,
        member.display_name(),
        member.user.id
    );

    Ok(())
}
