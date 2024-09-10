use thiserror::Error;
use uuid::Uuid;

use crate::core::session::{Session, SessionKey, SessionName};

use super::{connection, DatabaseError};

pub mod member;
pub mod tech;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Database error: `{0}`")]
    Database(#[from] DatabaseError),
}

pub async fn create_session(name: SessionName) -> Result<Session, SessionError> {
    let key = SessionKey(Uuid::new_v4().to_string());

    sqlx::query!(
        r#"INSERT INTO session ( key, name ) VALUES ( ?1, ?2 )"#,
        key.0,
        name.0,
    )
    .execute(&mut *connection().await?)
    .await
    .map_err(DatabaseError::from)?;

    Ok(Session::builder().name(name).key(key).build())
}

pub async fn get_session(key: &SessionKey) -> Result<Session, SessionError> {
    let raw_session = sqlx::query!(r#"SELECT name FROM session WHERE key = ?1"#, key.0)
        .fetch_one(&mut *connection().await?)
        .await
        .map_err(DatabaseError::from)?;

    Ok(Session::builder()
        .name(SessionName(raw_session.name))
        .key(key.clone())
        .build())
}
