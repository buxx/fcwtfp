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

#[derive(Debug, Clone, PartialEq, EnumString, EnumIter, Display, Deserialize, Serialize)]
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
