use sqlx::{Pool, Sqlite};

use crate::{
    session::{
        city::{CitiesState, City, CityError, CityInState, CityState},
        member::MemberDiscordId,
        SessionKey,
    },
    DatabaseError,
};

use super::member::get_members;

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

pub async fn get_cities(
    pool: &Pool<Sqlite>,
    session_key: &SessionKey,
    member_discord_id: &MemberDiscordId,
) -> Result<Vec<City>, CityError> {
    Ok(sqlx::query!(
        r#"SELECT name FROM session_city WHERE session_key = ?1 AND session_member_discord_id = ?2"#,
        session_key.0,
        member_discord_id.0
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)?.iter().map(|raw_city| City::builder().name(raw_city.name.to_string()).build()).collect())
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

pub async fn remove_city(
    pool: &Pool<Sqlite>,
    session_key: &SessionKey,
    member_discord_id: &MemberDiscordId,
    city_name: &str,
) -> Result<(), CityError> {
    sqlx::query!(
            r#"DELETE FROM session_city WHERE session_key = ?1 AND session_member_discord_id = ?2 AND name = ?3"#,
            session_key.0,
            member_discord_id.0,
            city_name,
        )
        .execute(pool)
        .await
        .map_err(DatabaseError::from)?;

    Ok(())
}

pub async fn find_city_by_partial_name(
    pool: &Pool<Sqlite>,
    session_key: &SessionKey,
    partial_name: &str,
) -> Result<Vec<City>, CityError> {
    let partial_name_param = String::from("%") + partial_name;
    Ok(sqlx::query!(
        r#"SELECT name FROM session_city WHERE UPPER(name) LIKE ?1 AND session_key = ?2"#,
        partial_name_param,
        session_key.0,
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)?
    .iter()
    .map(|city_raw| City::builder().name(city_raw.name.to_string()).build())
    .collect())
}

pub async fn get_cities_state(
    pool: &Pool<Sqlite>,
    session_key: &SessionKey,
) -> Result<CitiesState, CityError> {
    let mut cities = vec![];

    for member in get_members(pool, session_key).await?.members() {
        let mut member_cities = vec![];
        for city in get_cities(pool, session_key, member.discord_id()).await? {
            let city_in_state = CityInState::new(city, CityState::Nothing);
            member_cities.push(city_in_state);
        }
        cities.push((member.clone(), member_cities));
    }

    Ok(CitiesState::builder().cities(cities).build())
}
