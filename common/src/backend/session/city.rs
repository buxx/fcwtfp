use sqlx::{Pool, Sqlite};

use crate::{
    session::{
        city::{City, CityError},
        member::MemberDiscordId,
        SessionKey,
    },
    DatabaseError,
};

pub async fn city_exist(
    pool: &Pool<Sqlite>,
    session_key: &SessionKey,
    member_discord_id: &MemberDiscordId,
    city_name: &str,
) -> Result<bool, CityError> {
    Ok(sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM session_city WHERE session_key = ?1 AND session_member_discord_id = ?2 AND name = ?3)",
    )
    .bind(&session_key.0)
    .bind(&member_discord_id.0)
    .bind(city_name)
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)?)
}

pub async fn add_city(
    pool: &Pool<Sqlite>,
    session_key: &SessionKey,
    member_discord_id: &MemberDiscordId,
    city_name: &str,
) -> Result<City, CityError> {
    sqlx::query!(
        r#"INSERT INTO session_city ( session_key, session_member_discord_id, name ) VALUES ( ?1, ?2, ?3 )"#,
        session_key.0,
        member_discord_id.0,
        city_name,
    )
    .execute(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(City::builder().name(city_name.to_string()).build())
}
