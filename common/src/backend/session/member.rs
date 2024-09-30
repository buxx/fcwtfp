use crate::session::{
    member::{Member, MemberDiscordId, MemberName, Members, MembersError},
    SessionKey,
};
use sqlx::{Pool, Sqlite};

use sqlx::{self};

use super::DatabaseError;

pub async fn get_members(pool: &Pool<Sqlite>, key: &SessionKey) -> Result<Members, MembersError> {
    let mut members = vec![];

    for raw_member in sqlx::query!(
        r#"SELECT name, discord_id FROM session_member WHERE session_key = ?1"#,
        key.0
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)?
    {
        members.push(
            Member::builder()
                .name(MemberName(raw_member.name))
                .discord_id(MemberDiscordId(raw_member.discord_id))
                .build(),
        );
    }

    Ok(Members(members))
}

async fn member_exist(
    pool: &Pool<Sqlite>,
    session_key: &SessionKey,
    member_discord_id: &MemberDiscordId,
) -> Result<bool, MembersError> {
    Ok(sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM session_member WHERE session_key = ?1 AND discord_id = ?2)",
    )
    .bind(&session_key.0)
    .bind(&member_discord_id.0)
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)?)
}

pub async fn ensure_session_member(
    pool: &Pool<Sqlite>,
    session_key: &SessionKey,
    member_discord_id: &MemberDiscordId,
    member_name: &MemberName,
) -> Result<Member, MembersError> {
    if !member_exist(pool, session_key, member_discord_id).await? {
        sqlx::query!(
            r#"INSERT INTO session_member ( session_key, name, discord_id ) VALUES ( ?1, ?2, ?3 )"#,
            session_key.0,
            member_name.0,
            member_discord_id.0,
        )
        .execute(pool)
        .await
        .map_err(DatabaseError::from)?;
    } else {
        let mut member = get_member(pool, session_key, member_discord_id).await?;
        if member.name().0 != member_name.0 {
            member.set_name(member_name.clone());
            sqlx::query!(r#"UPDATE session_member SET name = ?1"#, member_name.0,)
                .execute(pool)
                .await
                .map_err(DatabaseError::from)?;
        }
    }

    Ok(Member::builder()
        .name(member_name.clone())
        .discord_id(member_discord_id.clone())
        .build())
}

pub async fn get_member(
    pool: &Pool<Sqlite>,
    key: &SessionKey,
    member_discord_id: &MemberDiscordId,
) -> Result<Member, MembersError> {
    let raw_member = sqlx::query!(
        r#"SELECT name, discord_id FROM session_member WHERE session_key = ?1 AND discord_id = ?2"#,
        key.0,
        member_discord_id.0,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(Member::builder()
        .name(MemberName(raw_member.name))
        .discord_id(MemberDiscordId(raw_member.discord_id))
        .build())
}

// pub async fn add_member(
//     pool: &Pool<Sqlite>,
//     key: &SessionKey,
//     member_discord_id: MemberDiscordId,
//     member_name: MemberName,
// ) -> Result<(), MembersError> {
//     let raw_discord_id = member.discord_id().map(|v| v.0.clone());
//     if let Err(DatabaseError::UniqueConstraint) = sqlx::query!(
//         r#"INSERT INTO session_member ( session_key, name, discord_id ) VALUES ( ?1, ?2, ?3 )"#,
//         key.0,
//         member.name().0,
//         raw_discord_id,
//     )
//     .execute(pool)
//     .await
//     .map_err(DatabaseError::from)
//     {
//         return Err(MembersError::NameAlreadyExist);
//     };

//     Ok(())
// }

pub async fn remove_member(
    pool: &Pool<Sqlite>,
    key: SessionKey,
    member: Member,
) -> Result<(), MembersError> {
    sqlx::query!(
        r#"DELETE FROM session_member WHERE session_key = ?1 AND name = ?2"#,
        key.0,
        member.name().0
    )
    .execute(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(())
}
