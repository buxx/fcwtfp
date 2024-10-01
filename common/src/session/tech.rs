use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::{EnumIter, EnumString};
use thiserror::Error;

use crate::DatabaseError;

use super::member::{Member, MemberName, MembersError};

#[cfg(feature = "backend")]
use poise::ChoiceParameter;

#[derive(
    Debug, Clone, PartialEq, EnumString, EnumIter, Display, Deserialize, Serialize, Eq, Hash,
)]
#[cfg_attr(feature = "backend", derive(ChoiceParameter))]
pub enum Technology {
    AdvancedFlight,
    Alphabet,
    AmphibiousWarfare,
    Astronomy,
    AtomicTheory,
    Automobile,
    Avionics,
    Banking,
    BridgeBuilding,
    BronzeWorking,
    CeremonialBurial,
    Chemistry,
    Chivalry,
    CodeOfLaws,
    CombinedArms,
    Combustion,
    Combustion2,
    Communism,
    Computers,
    Conscription,
    Construction,
    Currency,
    Democracy,
    Economics,
    Electricity,
    Electronics,
    Engineering,
    Environmentalism,
    Espionage,
    Explosives,
    Feudalism,
    Flight,
    Flight2,
    FusionPower,
    GuerillaWarfare,
    Gunpowder,
    HorsebackRiding,
    Industrialization,
    Invention,
    IronWorking,
    Laser,
    Leadership,
    Literacy,
    MachineTools,
    Magnetism,
    MapMaking,
    Masonry,
    MassProduction,
    Mathematics,
    Mechanization,
    Medicine,
    Metallurgy,
    Microbiology,
    Miniaturization,
    MobileWarfare,
    Monarchy,
    Monotheism,
    Mysticism,
    Nationalism,
    Navigation,
    NuclearFission,
    NuclearPower,
    Philosophy,
    Physics,
    Plastics,
    Polytheism,
    Pottery,
    Radar,
    Radio,
    Radio2,
    Railroad,
    Recycling,
    Refining,
    Refrigeration,
    Robotics,
    Rocketry,
    Sanitation,
    Seafaring,
    SpaceFlight,
    SpaceFlight2,
    Stealth,
    SteamEngine,
    Steel,
    Superconductors,
    Tactics,
    TheCorporation,
    TheRepublic,
    TheWheel,
    Theocracy,
    Theology,
    TheoryOfGravity,
    Trade,
    University,
    WarriorCode,
    Writing,
}

#[derive(Debug, Clone, PartialEq, EnumString, EnumIter, Display, Deserialize, Serialize)]
#[cfg_attr(feature = "backend", derive(ChoiceParameter))]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TechnologyInState(Technology, Vec<Member>);

impl TechnologyInState {
    pub fn new(technology: Technology, members: Vec<Member>) -> Self {
        Self(technology, members)
    }

    pub fn technology_name(&self) -> &str {
        self.0.name()
    }

    pub fn member_names(&self) -> Vec<&MemberName> {
        self.1
            .iter()
            .map(|member| member.name())
            .collect::<Vec<&MemberName>>()
    }
}

// #[builder]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TechnologiesState {
    done: Vec<TechnologyInState>,
    search: Vec<TechnologyInState>,
}

impl TechnologiesState {
    pub fn done(&self) -> &[TechnologyInState] {
        &self.done
    }

    pub fn search(&self) -> &[TechnologyInState] {
        &self.search
    }

    pub fn add_search(&mut self, value: TechnologyInState) {
        self.search.push(value)
    }

    pub fn add_done(&mut self, value: TechnologyInState) {
        self.done.push(value)
    }
}
