use crate::{
    backend::connection,
    core::session::{
        member::{Member, MemberDiscordId, MemberName, Members, MembersError},
        SessionKey,
    },
};

use sqlx::{self};

use super::DatabaseError;

pub async fn get_members(key: &SessionKey) -> Result<Members, MembersError> {
    let mut members = vec![];

    for raw_member in sqlx::query!(
        r#"SELECT name, discord_id FROM session_member WHERE session_key = ?1"#,
        key.0
    )
    .fetch_all(&mut *connection().await?)
    .await
    .map_err(DatabaseError::from)?
    {
        members.push(
            Member::builder()
                .name(MemberName(raw_member.name))
                .maybe_discord_id(raw_member.discord_id.map(MemberDiscordId))
                .build(),
        );
    }

    Ok(Members(members))
}

pub async fn add_member(key: SessionKey, member: Member) -> Result<(), MembersError> {
    let raw_discord_id = member.discord_id().map(|v| v.0.clone());
    if let Err(DatabaseError::UniqueConstraint) = sqlx::query!(
        r#"INSERT INTO session_member ( session_key, name, discord_id ) VALUES ( ?1, ?2, ?3 )"#,
        key.0,
        member.name().0,
        raw_discord_id,
    )
    .execute(&mut *connection().await?)
    .await
    .map_err(DatabaseError::from)
    {
        return Err(MembersError::NameAlreadyExist);
    };

    Ok(())
}

pub async fn remove_member(key: SessionKey, member: Member) -> Result<(), MembersError> {
    sqlx::query!(
        r#"DELETE FROM session_member WHERE session_key = ?1 AND name = ?2"#,
        key.0,
        member.name().0
    )
    .execute(&mut *connection().await?)
    .await
    .map_err(DatabaseError::from)?;

    Ok(())
}
