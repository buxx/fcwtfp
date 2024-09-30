use bon::builder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

use crate::DatabaseError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Members(pub Vec<Member>);

#[derive(Error, Debug)]
pub enum MembersError {
    #[error("Database error: `{0}`")]
    Database(#[from] DatabaseError),
    #[error("Already exist")]
    NameAlreadyExist,
}

impl Members {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn members(&self) -> &[Member] {
        &self.0
    }

    pub fn add_member(&mut self, member: Member) {
        self.0.push(member)
    }

    pub fn remove_member(&mut self, name: &MemberName) {
        self.0.retain(|m| m.name.0 != name.0)
    }
}

impl Display for Members {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|m| m.name.0.clone())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberName(pub String);
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberDiscordId(pub String);

#[builder]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Member {
    name: MemberName,
    discord_id: MemberDiscordId,
}

impl Member {
    pub fn name(&self) -> &MemberName {
        &self.name
    }

    pub fn discord_id(&self) -> &MemberDiscordId {
        &self.discord_id
    }

    pub fn set_name(&mut self, name: MemberName) {
        self.name = name;
    }
}
