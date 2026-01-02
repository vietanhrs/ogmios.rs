//! Transaction types for Cardano.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::primitives::*;
use super::scripts::{Script, Datum, Redeemer};
use super::certificates::Certificate;

/// A Cardano transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Transaction ID.
    pub id: TransactionId,
    /// Whether the transaction is valid.
    #[serde(default = "default_true")]
    pub valid: bool,
    /// Transaction inputs.
    #[serde(default)]
    pub inputs: Vec<TransactionInput>,
    /// Transaction outputs.
    #[serde(default)]
    pub outputs: Vec<TransactionOutput>,
    /// Collateral inputs (for Plutus transactions).
    #[serde(default)]
    pub collaterals: Vec<TransactionInput>,
    /// Collateral return output.
    #[serde(default)]
    pub collateral_return: Option<TransactionOutput>,
    /// Total collateral amount.
    #[serde(default)]
    pub total_collateral: Option<Lovelace>,
    /// Reference inputs.
    #[serde(default)]
    pub references: Vec<TransactionInput>,
    /// Transaction fee.
    #[serde(default)]
    pub fee: Option<Lovelace>,
    /// Validity interval start (slot).
    #[serde(default)]
    pub valid_from: Option<Slot>,
    /// Validity interval end (slot).
    #[serde(default)]
    pub valid_until: Option<Slot>,
    /// Certificates included in the transaction.
    #[serde(default)]
    pub certificates: Vec<Certificate>,
    /// Withdrawals from reward accounts.
    #[serde(default)]
    pub withdrawals: HashMap<RewardAccount, Lovelace>,
    /// Minted/burned assets.
    #[serde(default)]
    pub mint: Assets,
    /// Required signers (for Plutus).
    #[serde(default)]
    pub required_extra_signers: Vec<DigestBlake2b224>,
    /// Required scripts.
    #[serde(default)]
    pub required_extra_scripts: Vec<ScriptHash>,
    /// Network ID.
    #[serde(default)]
    pub network: Option<String>,
    /// Script integrity hash.
    #[serde(default)]
    pub script_integrity_hash: Option<DigestBlake2b256>,
    /// Witness set.
    #[serde(default)]
    pub witnesses: Option<Witnesses>,
    /// Metadata.
    #[serde(default)]
    pub metadata: Option<Metadata>,
    /// CBOR representation (hex-encoded).
    #[serde(default)]
    pub cbor: Option<String>,
    /// Proposals (Conway era).
    #[serde(default)]
    pub proposals: Vec<serde_json::Value>,
    /// Votes (Conway era).
    #[serde(default)]
    pub votes: Vec<serde_json::Value>,
}

fn default_true() -> bool {
    true
}

/// A transaction input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInput {
    /// Transaction ID containing the output.
    pub transaction: TransactionOutputReference,
}

/// Reference to a transaction output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionOutputReference {
    /// Transaction ID.
    pub id: TransactionId,
    /// Output index.
    pub index: u32,
}

impl TransactionOutputReference {
    pub fn new(id: impl Into<String>, index: u32) -> Self {
        Self { id: id.into(), index }
    }
}

/// A transaction output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionOutput {
    /// Output address.
    pub address: Address,
    /// Output value.
    pub value: Value,
    /// Datum hash (Alonzo style).
    #[serde(default)]
    pub datum_hash: Option<DatumHash>,
    /// Inline datum (Babbage style).
    #[serde(default)]
    pub datum: Option<Datum>,
    /// Reference script.
    #[serde(default)]
    pub script: Option<Script>,
}

/// UTXO - a transaction output with its reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Utxo {
    /// Transaction reference.
    pub transaction: TransactionOutputReference,
    /// The output.
    #[serde(flatten)]
    pub output: TransactionOutput,
}

/// Witness set for a transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Witnesses {
    /// Key witnesses.
    #[serde(default)]
    pub keys: Vec<KeyWitness>,
    /// Script witnesses.
    #[serde(default)]
    pub scripts: HashMap<ScriptHash, Script>,
    /// Bootstrap witnesses (Byron).
    #[serde(default)]
    pub bootstrap: Vec<BootstrapWitness>,
    /// Datums.
    #[serde(default)]
    pub datums: HashMap<DatumHash, Datum>,
    /// Redeemers.
    #[serde(default)]
    pub redeemers: Vec<Redeemer>,
}

/// A key witness (signature).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyWitness {
    /// Verification key.
    pub key: VerificationKey,
    /// Signature.
    pub signature: Signature,
}

/// Bootstrap witness for Byron-era transactions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapWitness {
    /// Verification key.
    pub key: VerificationKey,
    /// Chain code.
    pub chain_code: String,
    /// Address attributes.
    pub address_attributes: String,
    /// Signature.
    pub signature: Signature,
}

/// Transaction metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Metadata labels and values.
    #[serde(default)]
    pub labels: MetadataLabels,
    /// Hash of the metadata.
    #[serde(default)]
    pub hash: Option<DigestBlake2b256>,
}

/// Input source type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InputSource {
    /// Input is from spending.
    Inputs,
    /// Input is from collateral.
    Collaterals,
}

/// Transaction evaluation result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationResult {
    /// Validator index.
    pub validator: ValidatorIndex,
    /// Execution budget used.
    pub budget: ExUnits,
}

/// Validator index in a transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorIndex {
    /// Purpose of the script.
    pub purpose: ScriptPurpose,
    /// Index.
    pub index: u32,
}

/// Purpose of a script execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScriptPurpose {
    /// Spending a UTXO.
    Spend,
    /// Minting/burning tokens.
    Mint,
    /// Publishing a certificate.
    Publish,
    /// Withdrawing rewards.
    Withdraw,
    /// Proposing governance action (Conway).
    Propose,
    /// Voting (Conway).
    Vote,
}

/// Execution units (memory and CPU steps).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExUnits {
    /// Memory units.
    pub memory: u64,
    /// CPU steps.
    pub cpu: u64,
}

impl ExUnits {
    pub fn new(memory: u64, cpu: u64) -> Self {
        Self { memory, cpu }
    }
}
