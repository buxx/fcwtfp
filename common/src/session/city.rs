use bon::builder;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::{EnumIter, EnumString};
use thiserror::Error;

use crate::DatabaseError;

use super::member::{Member, MembersError};

#[derive(Error, Debug)]
pub enum CityError {
    #[error("Database error: `{0}`")]
    Database(#[from] DatabaseError),
    #[error("Already exist")]
    NameAlreadyExist,
    #[error("Members error: `{0}`")]
    Member(#[from] MembersError),
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

#[derive(Debug, Clone, PartialEq, EnumString, EnumIter, Display, Deserialize, Serialize)]
pub enum CityState {
    Nothing,
    TradeRoutePlannedOrEstablished,
}

#[derive(Error, Debug)]
pub enum CityStateError {
    #[error("Database error: `{0}`")]
    Database(#[from] DatabaseError),
    #[error("City error: `{0}`")]
    City(#[from] CityError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CityInState(City, CityState);

impl CityInState {
    pub fn new(city: City, state: CityState) -> Self {
        Self(city, state)
    }

    pub fn city_name(&self) -> &str {
        self.0.name()
    }

    pub fn state(&self) -> &CityState {
        &self.1
    }
}

#[builder]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CitiesState {
    cities: Vec<(Member, Vec<CityInState>)>,
}

impl CitiesState {
    pub fn cities(&self) -> &Vec<(Member, Vec<CityInState>)> {
        &self.cities
    }

    pub fn add_cities(&mut self, value: (Member, Vec<CityInState>)) {
        self.cities.push(value)
    }
}
