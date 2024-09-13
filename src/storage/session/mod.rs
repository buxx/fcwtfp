use redb::TableDefinition;
use thiserror::Error;
use uuid::Uuid;

use crate::session::{member::MembersError, Session, SessionKey, SessionName};

use super::DatabaseError;

pub mod member;

const SESSION: TableDefinition<&str, &str> = TableDefinition::new("session");

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Database error: `{0}`")]
    Database(#[from] DatabaseError),
    #[error("Session members error: `{0}`")]
    Members(#[from] MembersError),
}

pub fn create_session(name: SessionName) -> Result<Session, SessionError> {
    let key = SessionKey(Uuid::new_v4().to_string());
    let write_txn = super::db()?.begin_write().map_err(DatabaseError::from)?;
    {
        let mut table = write_txn.open_table(SESSION).map_err(DatabaseError::from)?;
        table
            .insert(key.0.as_str(), &name.0.as_str())
            .map_err(DatabaseError::from)?;
    }
    write_txn.commit().map_err(DatabaseError::from)?;

    Ok(Session::builder().name(name).key(key).build())
}

pub fn get_session(key: &SessionKey) -> Result<Session, SessionError> {
    let read_txn = super::db()?.begin_read().map_err(DatabaseError::from)?;
    let table = read_txn.open_table(SESSION).map_err(DatabaseError::from)?;
    let binding = table
        .get(key.0.as_str())
        .map_err(DatabaseError::from)?
        .unwrap();
    let name = binding.value();
    let name = SessionName(name.to_string());

    Ok(Session::builder().name(name).key(key.clone()).build())
}
