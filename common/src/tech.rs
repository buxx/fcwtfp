use poise::ChoiceParameter;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use strum_macros::{EnumIter, EnumString};

#[derive(
    Debug, Clone, PartialEq, EnumString, EnumIter, Display, Deserialize, Serialize, ChoiceParameter,
)]
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
