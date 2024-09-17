use bon::builder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

use crate::core::DatabaseError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Members(pub Vec<Member>);

#[derive(Error, Debug)]
pub enum MembersError {
    #[error("Database error: `{0}`")]
    Database(#[from] DatabaseError),
}

impl Members {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn from_raw(raw_members: &str) -> Result<Self, MembersError> {
        let mut members = vec![];

        for raw_member in raw_members.split(';') {
            let member_builder = Member::builder();
            let pieces = raw_member.split(',').collect::<Vec<&str>>();

            let name = MemberName(pieces.first().unwrap().to_string());
            let member_builder = member_builder
                .name(name)
                .maybe_discord_id(pieces.get(1).map(|v| MemberDiscordId(v.to_string())));

            members.push(member_builder.build());
        }

        Ok(Self(members))
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

    pub fn to_raw(&self) -> String {
        self.0
            .iter()
            .map(|m| {
                format!(
                    "{},{}",
                    m.name.0,
                    m.discord_id
                        .as_ref()
                        .unwrap_or(&MemberDiscordId("".to_string()))
                        .0
                )
            })
            .collect::<Vec<String>>()
            .join(";")
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
    discord_id: Option<MemberDiscordId>,
}

impl Member {
    pub fn name(&self) -> &MemberName {
        &self.name
    }

    pub fn discord_id(&self) -> Option<&MemberDiscordId> {
        self.discord_id.as_ref()
    }
}
