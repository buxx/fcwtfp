use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqlitePool;
use sqlx::{Error as SqlxError, Sqlite};

use crate::core::DatabaseError;

pub mod session;

const DB_FILE_PATH: &str = "sqlite://db.sqlite";

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

#[cfg(feature = "server")]
pub async fn connection() -> Result<PoolConnection<Sqlite>, DatabaseError> {
    Ok(SqlitePool::connect(DB_FILE_PATH).await?.acquire().await?)
}
