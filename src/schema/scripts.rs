//! Script and datum types for Cardano.

use serde::{Deserialize, Serialize};
use super::primitives::*;
use super::transaction::ExUnits;

/// A Cardano script.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "language", rename_all = "camelCase")]
pub enum Script {
    /// Native script (multi-sig, timelocks).
    Native {
        #[serde(rename = "json")]
        script: NativeScript,
        #[serde(default)]
        cbor: Option<String>,
    },
    /// Plutus V1 script.
    #[serde(rename = "plutus:v1")]
    PlutusV1 {
        cbor: String,
    },
    /// Plutus V2 script.
    #[serde(rename = "plutus:v2")]
    PlutusV2 {
        cbor: String,
    },
    /// Plutus V3 script.
    #[serde(rename = "plutus:v3")]
    PlutusV3 {
        cbor: String,
    },
}

/// Native script types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "clause", rename_all = "camelCase")]
pub enum NativeScript {
    /// Signature required.
    #[serde(rename = "signature")]
    Signature {
        from: DigestBlake2b224,
    },
    /// All scripts must validate.
    #[serde(rename = "all")]
    All {
        from: Vec<NativeScript>,
    },
    /// Any script must validate.
    #[serde(rename = "any")]
    Any {
        from: Vec<NativeScript>,
    },
    /// At least M of N scripts must validate.
    #[serde(rename = "some")]
    Some {
        #[serde(rename = "atLeast")]
        at_least: u32,
        from: Vec<NativeScript>,
    },
    /// Valid after slot.
    #[serde(rename = "after")]
    After {
        slot: Slot,
    },
    /// Valid before slot.
    #[serde(rename = "before")]
    Before {
        slot: Slot,
    },
}

/// Plutus language versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "plutus:v1")]
    PlutusV1,
    #[serde(rename = "plutus:v2")]
    PlutusV2,
    #[serde(rename = "plutus:v3")]
    PlutusV3,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::PlutusV1 => "plutus:v1",
            Language::PlutusV2 => "plutus:v2",
            Language::PlutusV3 => "plutus:v3",
        }
    }
}

/// Datum (inline or hash reference).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Datum {
    /// CBOR-encoded datum.
    Cbor(String),
    /// Datum value (parsed).
    Value(serde_json::Value),
}

/// Redeemer for script execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Redeemer {
    /// Script purpose.
    pub purpose: RedeemerPurpose,
    /// Redeemer data.
    pub datum: Datum,
    /// Execution budget.
    pub execution_units: ExUnits,
}

/// Redeemer purpose (what the script is validating).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "purpose", rename_all = "camelCase")]
pub enum RedeemerPurpose {
    /// Spending a UTXO.
    Spend {
        #[serde(rename = "outputReference")]
        output_reference: OutputReference,
    },
    /// Minting tokens.
    Mint {
        policy: PolicyId,
    },
    /// Publishing a certificate.
    Publish {
        #[serde(rename = "certificateIndex")]
        certificate_index: u32,
    },
    /// Withdrawing from a reward account.
    Withdraw {
        #[serde(rename = "rewardAccount")]
        reward_account: RewardAccount,
    },
    /// Proposing (Conway).
    Propose {
        #[serde(rename = "proposalIndex")]
        proposal_index: u32,
    },
    /// Voting (Conway).
    Vote {
        voter: serde_json::Value,
    },
}

/// Output reference for redeemer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputReference {
    /// Transaction ID.
    #[serde(rename = "transaction")]
    pub transaction_id: TransactionId,
    /// Output index.
    pub index: u32,
}

/// Script reference in a UTXO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptReference {
    /// Script hash.
    pub hash: ScriptHash,
    /// Script language.
    pub language: Language,
    /// CBOR-encoded script (optional).
    #[serde(default)]
    pub cbor: Option<String>,
}
