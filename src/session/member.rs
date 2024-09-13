use std::fmt::Display;

use bon::builder;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    session::{SessionHeader, SessionNavBar},
    RootNavBar,
};

#[cfg(feature = "server")]
use crate::storage::{self};

use super::SessionKey;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Members {
    key: SessionKey,
    members: Vec<Member>,
}

#[derive(Error, Debug)]
pub enum MembersError {}

impl Members {
    pub fn empty(key: SessionKey) -> Self {
        Self {
            key,
            members: vec![],
        }
    }

    pub fn from_raw(key: SessionKey, raw_members: &str) -> Result<Self, MembersError> {
        let mut members = vec![];

        for raw_member in raw_members.split(';') {
            let member_builder = Member::builder();
            let pieces = raw_member.split(',').collect::<Vec<&str>>();

            let name = MemberName(pieces.first().unwrap().to_string());
            let member_builder = member_builder.name(name);
            // if let Some(discord_id) = pieces.get(1).map(|v| MemberDiscordId(v.to_string())) {
            //     member_builder = member_builder.discord_id(discord_id)
            // };

            members.push(member_builder.build());
        }

        Ok(Self { key, members })
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.push(member)
    }

    pub fn remove_member(&mut self, member: Member) {
        self.members.retain(|m| m.name.0 != member.name.0)
    }

    pub fn to_raw(&self) -> String {
        self.members
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
                .members
                .iter()
                .map(|m| m.name.0.clone())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberName(String);
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberDiscordId(String);

#[builder]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Member {
    name: MemberName,
    discord_id: Option<MemberDiscordId>,
}

#[component]
pub fn MembersHome(session_key: ReadOnlySignal<String>) -> Element {
    rsx! {
        RootNavBar {}
        SessionHeader { session_key }
        SessionNavBar { session_key }
        h1 { "Members" }
        AddMember {session_key}
    }
}

#[component]
pub fn AddMember(session_key: ReadOnlySignal<String>) -> Element {
    let mut name = use_signal(|| String::from(""));

    rsx! {
        form {
            onsubmit: move |_| async move {
                add_member(session_key.to_string(), name.to_string()).await.unwrap();
            },
            label { r#for: "name", "Name" },
            input {
                value: "{name}",
                id: "name",
                oninput: move |evt| name.set(evt.value().clone()),
            }
            input { r#type: "submit", value: "Add" },
        }
    }
}

#[server(CreateSession)]
async fn add_member(session_key: String, name: String) -> Result<(), ServerFnError> {
    storage::session::member::add_member(
        SessionKey(session_key),
        Member::builder().name(MemberName(name)).build(),
    )?;
    Ok(())
}
