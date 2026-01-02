//! Era types for Cardano.

use serde::{Deserialize, Serialize};
use super::primitives::*;

/// Cardano era names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Era {
    Byron,
    Shelley,
    Allegra,
    Mary,
    Alonzo,
    Babbage,
    Conway,
}

impl Era {
    /// Get the era as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Era::Byron => "byron",
            Era::Shelley => "shelley",
            Era::Allegra => "allegra",
            Era::Mary => "mary",
            Era::Alonzo => "alonzo",
            Era::Babbage => "babbage",
            Era::Conway => "conway",
        }
    }
}

impl std::fmt::Display for Era {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Eras that have genesis configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EraWithGenesis {
    Byron,
    Shelley,
    Alonzo,
    Conway,
}

impl EraWithGenesis {
    /// Get the era as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            EraWithGenesis::Byron => "byron",
            EraWithGenesis::Shelley => "shelley",
            EraWithGenesis::Alonzo => "alonzo",
            EraWithGenesis::Conway => "conway",
        }
    }
}

/// Era summary information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EraSummary {
    /// Start of the era.
    pub start: EraBound,
    /// End of the era (None if current).
    #[serde(default)]
    pub end: Option<EraBound>,
    /// Era parameters.
    pub parameters: EraParameters,
}

/// Era boundary point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EraBound {
    /// Slot number.
    pub slot: Slot,
    /// Epoch number.
    pub epoch: Epoch,
    /// Time since system start.
    pub time: RelativeTime,
}

/// Parameters for an era.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EraParameters {
    /// Length of an epoch in slots.
    pub epoch_length: u64,
    /// Duration of a slot in seconds.
    pub slot_length: RelativeTime,
    /// Safe zone (slots before era end to stop accepting certain operations).
    #[serde(default)]
    pub safe_zone: Option<u64>,
}

/// Era start information from ledger queries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EraStart {
    /// Time since system start.
    pub time: RelativeTime,
    /// Slot number.
    pub slot: Slot,
    /// Epoch number.
    pub epoch: Epoch,
}
