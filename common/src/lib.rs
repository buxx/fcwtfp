use thiserror::Error;

#[cfg(feature = "backend")]
pub mod backend;
pub mod session;

#[cfg(feature = "backend")]
const DB_FILE_PATH: &str = "sqlite://db.sqlite";

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: `{0}`")]
    Database(String),
    #[error("Unique constraint error")]
    UniqueConstraint,
}
