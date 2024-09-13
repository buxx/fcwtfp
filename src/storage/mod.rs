pub mod session;
pub mod tech;

use redb::{
    CommitError, Database, DatabaseError as RedbDatabaseError, StorageError, TableError,
    TransactionError,
};
use thiserror::Error;

const DB_FILE_PATH: &str = "db.redb";

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: `{0}`")]
    Database(#[from] RedbDatabaseError),
    #[error("Transaction error: `{0}`")]
    Transaction(#[from] TransactionError),
    #[error("Commit error: `{0}`")]
    Commit(#[from] CommitError),
    #[error("Storage error: `{0}`")]
    Storage(#[from] StorageError),
    #[error("Table error: `{0}`")]
    Table(#[from] TableError),
}

pub fn db() -> Result<Database, DatabaseError> {
    Ok(Database::create(DB_FILE_PATH)?)
}
