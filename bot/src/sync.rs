use common::{
    backend::{
        pool,
        session::{ensure_session, get_session, member::ensure_session_member, SessionError},
    },
    session::{
        member::{MemberDiscordId, MemberName, MembersError},
        SessionDiscordId, SessionKey, SessionName,
    },
    DatabaseError,
};
use poise::serenity_prelude::{self as serenity, futures::StreamExt};
use sqlx::{Pool, Sqlite};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Database opening: {0}")]
    Database(#[from] DatabaseError),
    #[error("Session synchronisation: {0:?}")]
    SyncGuilds(Vec<(serenity::GuildId, SyncGuildError)>),
    #[error("Members synchronisation: {0:?}")]
    SyncMembers(Vec<(serenity::GuildId, Option<serenity::UserId>, SyncMemberError)>),
}

#[derive(Error, Debug)]
pub enum SyncMemberError {
    #[error("Session: {0}")]
    Session(#[from] SessionError),
    #[error("Serenity: {0}")]
    Serenity(#[from] serenity::Error),
    #[error("Serenity: {0}")]
    Member(#[from] MembersError),
}

#[derive(Error, Debug)]
pub enum SyncGuildError {
    #[error("Serenity: {0}")]
    SerenityError(#[from] serenity::Error),
    #[error("Serenity: {0}")]
    SessionError(#[from] SessionError),
}

pub async fn sync_all(ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), SyncError> {
    let pool = pool().await?;

    sync_guilds(&pool, ctx, ready).await?;
    sync_members(&pool, ctx, ready).await?;

    Ok(())
}

pub async fn sync_new_member(
    _ctx: &serenity::Context,
    new_member: &serenity::Member,
) -> Result<(), SyncError> {
    let pool = pool().await?;
    match get_session(&pool, &SessionDiscordId(new_member.guild_id.to_string())).await {
        Ok(session) => {
            if let Err(error) = sync_member(&pool, session.key(), new_member).await {
                //
                return Err(SyncError::SyncMembers(vec![(
                    new_member.guild_id,
                    None,
                    error,
                )]));
            }
        }
        Err(error) => {
            //
            return Err(SyncError::SyncMembers(vec![(
                new_member.guild_id,
                None,
                SyncMemberError::Session(error),
            )]));
        }
    }

    Ok(())
}

async fn sync_guilds(
    pool: &Pool<Sqlite>,
    ctx: &serenity::Context,
    ready: &serenity::Ready,
) -> Result<(), SyncError> {
    let mut errors: Vec<(serenity::GuildId, SyncGuildError)> = vec![];

    for guild in &ready.guilds {
        match serenity::Guild::get(ctx, guild.id).await {
            Ok(guild) => {
                if let Err(error) = sync_guild(pool, &guild).await {
                    errors.push((guild.id, error));
                }
            }
            Err(error) => {
                errors.push((guild.id, SyncGuildError::from(error)));
            }
        };
    }

    if !errors.is_empty() {
        return Err(SyncError::SyncGuilds(errors));
    }

    Ok(())
}

async fn sync_members(
    pool: &Pool<Sqlite>,
    ctx: &serenity::Context,
    ready: &serenity::Ready,
) -> Result<(), SyncError> {
    let mut errors: Vec<(serenity::GuildId, Option<serenity::UserId>, SyncMemberError)> = vec![];

    for guild in &ready.guilds {
        match get_session(pool, &SessionDiscordId(guild.id.to_string())).await {
            Ok(session) => {
                let mut members =
                    serenity::MembersIter::<serenity::Http>::stream(&ctx, guild.id).boxed();

                while let Some(member_result) = members.next().await {
                    match member_result {
                        Ok(member) => {
                            if !member.user.bot {
                                if let Err(error) = sync_member(pool, session.key(), &member).await
                                {
                                    errors.push((guild.id, Some(member.user.id), error));
                                }
                            }
                        }
                        Err(error) => {
                            errors.push((guild.id, None, SyncMemberError::from(error)));
                        }
                    }
                }
            }
            Err(error) => {
                errors.push((guild.id, None, SyncMemberError::from(error)));
            }
        };
    }

    if !errors.is_empty() {
        return Err(SyncError::SyncMembers(errors));
    }

    Ok(())
}

async fn sync_guild(
    pool: &Pool<Sqlite>,
    guild: &serenity::PartialGuild,
) -> Result<(), SyncGuildError> {
    ensure_session(
        pool,
        &SessionDiscordId(guild.id.to_string()),
        &SessionName(guild.name.clone()),
    )
    .await?;
    Ok(())
}

async fn sync_member(
    pool: &Pool<Sqlite>,
    session_key: &SessionKey,
    member: &serenity::Member,
) -> Result<(), SyncMemberError> {
    ensure_session_member(
        pool,
        session_key,
        &MemberDiscordId(member.user.id.to_string()),
        &MemberName(member.nick.clone().unwrap_or(member.user.name.clone())),
    )
    .await?;

    Ok(())
}
