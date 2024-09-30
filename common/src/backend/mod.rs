use sqlx::{pool::PoolConnection, Error as SqlxError, Sqlite, SqlitePool};

use crate::{DatabaseError, DB_FILE_PATH};

pub mod session;

// Sqlx is only compiled for backend, so convert sqlx error in string
impl From<SqlxError> for DatabaseError {
    fn from(value: SqlxError) -> Self {
        if let SqlxError::Database(db_error) = &value {
            if db_error.is_unique_violation() {
                return Self::UniqueConstraint;
            }
        }

        Self::Database(value.to_string())
    }
}

pub async fn connection() -> Result<PoolConnection<Sqlite>, DatabaseError> {
    Ok(SqlitePool::connect(DB_FILE_PATH).await?.acquire().await?)
}
