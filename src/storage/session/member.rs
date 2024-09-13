use redb::{TableDefinition, TableError};

use crate::{
    session::{
        member::{Member, Members},
        SessionKey,
    },
    storage::db,
};

use super::{DatabaseError, SessionError};

const SESSION_MEMBER: TableDefinition<&str, &str> = TableDefinition::new("session_member");

pub fn get_session_members(key: &SessionKey) -> Result<Members, SessionError> {
    let read_txn = db()?.begin_read().map_err(DatabaseError::from)?;
    let table = match read_txn.open_table(SESSION_MEMBER) {
        Ok(table) => table,
        Err(TableError::TableDoesNotExist(_)) => return Ok(Members::empty(key.clone())),
        Err(error) => return Err(SessionError::Database(DatabaseError::from(error))),
    };
    let binding = table
        .get(key.0.as_str())
        .map_err(DatabaseError::from)?
        .unwrap();
    let raw_members = binding.value();
    let members = Members::from_raw(key.clone(), raw_members)?;

    Ok(members)
}

pub fn add_member(key: SessionKey, member: Member) -> Result<(), SessionError> {
    let mut session_members = get_session_members(&key)?;
    session_members.add_member(member);

    let write_txn = db()?.begin_write().map_err(DatabaseError::from)?;
    {
        let mut table = write_txn
            .open_table(SESSION_MEMBER)
            .map_err(DatabaseError::from)?;
        table
            .insert(key.0.as_str(), session_members.to_raw().as_str())
            .map_err(DatabaseError::from)?;
    }
    write_txn.commit().map_err(DatabaseError::from)?;

    Ok(())
}

pub fn remove_session_member(key: SessionKey, member: Member) -> Result<(), SessionError> {
    let mut session_members = get_session_members(&key)?;
    session_members.remove_member(member);

    let write_txn = db()?.begin_write().map_err(DatabaseError::from)?;
    let mut table = write_txn
        .open_table(SESSION_MEMBER)
        .map_err(DatabaseError::from)?;
    table
        .insert(key.0.as_str(), session_members.to_raw().as_str())
        .map_err(DatabaseError::from)?
        .unwrap();

    Ok(())
}
