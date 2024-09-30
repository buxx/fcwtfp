use bon::builder;
use serde::{Deserialize, Serialize};

pub mod member;
pub mod tech;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionName(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionDiscordId(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionKey(pub String);

#[builder]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    name: SessionName,
    key: SessionKey,
}

impl Session {
    pub fn name(&self) -> &SessionName {
        &self.name
    }

    pub fn key(&self) -> &SessionKey {
        &self.key
    }

    pub fn set_name(&mut self, name: SessionName) {
        self.name = name;
    }
}
