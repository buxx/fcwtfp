use common::tech::Technology;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::{EnumIter, EnumString};
use thiserror::Error;

use super::member::MembersError;
use crate::core::DatabaseError;

#[derive(Debug, Clone, PartialEq, EnumString, EnumIter, Display, Deserialize, Serialize)]
pub enum State {
    Researching,
    Done,
    Cancel,
}

#[derive(Error, Debug)]
pub enum TechnologyStateError {
    #[error("Database error: `{0}`")]
    Database(#[from] DatabaseError),
    #[error("Session members error: `{0}`")]
    Members(#[from] MembersError),
}

// #[builder]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TechnologyState {
    done: Vec<Technology>,
    search: Vec<Technology>,
}

impl TechnologyState {
    pub fn done(&self) -> &[Technology] {
        &self.done
    }

    pub fn search(&self) -> &[Technology] {
        &self.search
    }

    pub fn add_search(&mut self, technology: Technology) {
        self.search.push(technology)
    }

    pub fn add_done(&mut self, technology: Technology) {
        self.done.push(technology)
    }
}
