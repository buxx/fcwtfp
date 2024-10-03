use bon::builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::DatabaseError;

#[derive(Error, Debug)]
pub enum CityError {
    #[error("Database error: `{0}`")]
    Database(#[from] DatabaseError),
    #[error("Already exist")]
    NameAlreadyExist,
}

#[builder]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct City {
    name: String,
}

impl City {
    pub fn name(&self) -> &str {
        &self.name
    }
}
