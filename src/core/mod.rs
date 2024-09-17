use thiserror::Error;

pub mod session;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: `{0}`")]
    Database(String),
}
