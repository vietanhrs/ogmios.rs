//! Protocol parameter types for Cardano.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::primitives::*;
use super::transaction::ExUnits;

/// Protocol parameters for Cardano.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolParameters {
    /// Minimum fee coefficient (per byte).
    pub min_fee_coefficient: u64,
    /// Minimum fee constant.
    pub min_fee_constant: AdaValue,
    /// Minimum fee for reference scripts.
    #[serde(default)]
    pub min_fee_reference_scripts: Option<MinFeeReferenceScripts>,
    /// Maximum block body size in bytes.
    pub max_block_body_size: BlockSize,
    /// Maximum block header size in bytes.
    pub max_block_header_size: BlockSize,
    /// Maximum transaction size in bytes.
    pub max_transaction_size: BlockSize,
    /// Stake key deposit.
    pub stake_credential_deposit: AdaValue,
    /// Pool registration deposit.
    pub stake_pool_deposit: AdaValue,
    /// Pool retirement epoch bound.
    pub stake_pool_retirement_epoch_bound: u64,
    /// Desired number of stake pools.
    pub desired_number_of_stake_pools: u64,
    /// Stake pool pledge influence (a0).
    pub stake_pool_pledge_influence: Ratio,
    /// Monetary expansion (rho).
    pub monetary_expansion: Ratio,
    /// Treasury expansion (tau).
    pub treasury_expansion: Ratio,
    /// Protocol version.
    pub version: ProtocolVersion,
    /// Minimum stake pool cost.
    pub min_stake_pool_cost: AdaValue,
    /// Extra entropy (deprecated, always neutral).
    #[serde(default)]
    pub extra_entropy: Option<Nonce>,
    /// Minimum UTXO deposit coefficient (lovelace per byte).
    #[serde(default)]
    pub min_utxo_deposit_coefficient: Option<u64>,
    /// Minimum UTXO deposit constant.
    #[serde(default)]
    pub min_utxo_deposit_constant: Option<AdaValue>,
    /// Plutus cost models.
    #[serde(default)]
    pub plutus_cost_models: Option<CostModels>,
    /// Script execution prices.
    #[serde(default)]
    pub script_execution_prices: Option<ScriptExecutionPrices>,
    /// Maximum execution units per transaction.
    #[serde(default)]
    pub max_execution_units_per_transaction: Option<ExUnits>,
    /// Maximum execution units per block.
    #[serde(default)]
    pub max_execution_units_per_block: Option<ExUnits>,
    /// Maximum collateral inputs.
    #[serde(default)]
    pub max_collateral_inputs: Option<u64>,
    /// Collateral percentage.
    #[serde(default)]
    pub collateral_percentage: Option<u64>,
    /// Maximum value size in bytes.
    #[serde(default)]
    pub max_value_size: Option<BlockSize>,
    /// DRep deposit (Conway).
    #[serde(default)]
    pub delegate_representative_deposit: Option<AdaValue>,
    /// DRep max idle time (Conway).
    #[serde(default)]
    pub delegate_representative_max_idle_time: Option<u64>,
    /// Governance action deposit (Conway).
    #[serde(default)]
    pub governance_action_deposit: Option<AdaValue>,
    /// Governance action lifetime (Conway).
    #[serde(default)]
    pub governance_action_lifetime: Option<u64>,
    /// Constitutional committee min size (Conway).
    #[serde(default)]
    pub constitutional_committee_min_size: Option<u64>,
    /// Constitutional committee max term length (Conway).
    #[serde(default)]
    pub constitutional_committee_max_term_length: Option<u64>,
    /// Stake pool voting thresholds (Conway).
    #[serde(default)]
    pub stake_pool_voting_thresholds: Option<StakePoolVotingThresholds>,
    /// DRep voting thresholds (Conway).
    #[serde(default)]
    pub delegate_representative_voting_thresholds: Option<DelegateRepresentativeVotingThresholds>,
}

/// Minimum fee for reference scripts configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinFeeReferenceScripts {
    /// Base fee.
    pub base: f64,
    /// Fee range.
    pub range: u64,
    /// Multiplier.
    pub multiplier: f64,
}

/// Block size specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockSize {
    /// Size in bytes.
    pub bytes: NumberOfBytes,
}

/// Protocol version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolVersion {
    /// Major version.
    pub major: u32,
    /// Minor version.
    pub minor: u32,
    /// Patch version (optional).
    #[serde(default)]
    pub patch: Option<u32>,
}

/// Cost models for Plutus scripts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostModels {
    /// Plutus V1 cost model.
    #[serde(default, rename = "plutus:v1")]
    pub plutus_v1: Option<Vec<i64>>,
    /// Plutus V2 cost model.
    #[serde(default, rename = "plutus:v2")]
    pub plutus_v2: Option<Vec<i64>>,
    /// Plutus V3 cost model.
    #[serde(default, rename = "plutus:v3")]
    pub plutus_v3: Option<Vec<i64>>,
}

/// Script execution prices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptExecutionPrices {
    /// Memory price (lovelace per unit).
    pub memory: Ratio,
    /// CPU price (lovelace per step).
    pub cpu: Ratio,
}

/// Stake pool voting thresholds (Conway governance).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakePoolVotingThresholds {
    /// No confidence threshold.
    pub no_confidence: Ratio,
    /// Constitutional committee update threshold.
    pub constitutional_committee: ConstitutionalCommitteeThresholds,
    /// Hard fork initiation threshold.
    pub hard_fork_initiation: Ratio,
    /// Protocol parameters update threshold.
    #[serde(default)]
    pub protocol_parameters_update: Option<ProtocolParametersUpdateThresholds>,
}

/// Constitutional committee thresholds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstitutionalCommitteeThresholds {
    /// Default threshold.
    pub default: Ratio,
    /// State of no confidence threshold.
    pub state_of_no_confidence: Ratio,
}

/// Protocol parameters update thresholds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolParametersUpdateThresholds {
    /// Security threshold.
    pub security: Ratio,
}

/// DRep voting thresholds (Conway governance).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateRepresentativeVotingThresholds {
    /// No confidence threshold.
    pub no_confidence: Ratio,
    /// Constitution threshold.
    pub constitution: Ratio,
    /// Constitutional committee update threshold.
    pub constitutional_committee: ConstitutionalCommitteeThresholds,
    /// Hard fork initiation threshold.
    pub hard_fork_initiation: Ratio,
    /// Protocol parameters update thresholds.
    pub protocol_parameters_update: DRepProtocolParametersUpdateThresholds,
    /// Treasury withdrawal threshold.
    pub treasury_withdrawals: Ratio,
}

/// DRep protocol parameters update thresholds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DRepProtocolParametersUpdateThresholds {
    /// Network threshold.
    pub network: Ratio,
    /// Economic threshold.
    pub economic: Ratio,
    /// Technical threshold.
    pub technical: Ratio,
    /// Governance threshold.
    pub governance: Ratio,
}

/// Proposed protocol parameter updates.
pub type ProposedProtocolParameters = HashMap<DigestBlake2b224, PartialProtocolParameters>;

/// Partial protocol parameters for updates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PartialProtocolParameters {
    #[serde(default)]
    pub min_fee_coefficient: Option<u64>,
    #[serde(default)]
    pub min_fee_constant: Option<AdaValue>,
    #[serde(default)]
    pub max_block_body_size: Option<BlockSize>,
    #[serde(default)]
    pub max_block_header_size: Option<BlockSize>,
    #[serde(default)]
    pub max_transaction_size: Option<BlockSize>,
    #[serde(default)]
    pub stake_credential_deposit: Option<AdaValue>,
    #[serde(default)]
    pub stake_pool_deposit: Option<AdaValue>,
    #[serde(default)]
    pub stake_pool_retirement_epoch_bound: Option<u64>,
    #[serde(default)]
    pub desired_number_of_stake_pools: Option<u64>,
    #[serde(default)]
    pub stake_pool_pledge_influence: Option<Ratio>,
    #[serde(default)]
    pub monetary_expansion: Option<Ratio>,
    #[serde(default)]
    pub treasury_expansion: Option<Ratio>,
    #[serde(default)]
    pub version: Option<ProtocolVersion>,
    #[serde(default)]
    pub min_stake_pool_cost: Option<AdaValue>,
    #[serde(default)]
    pub plutus_cost_models: Option<CostModels>,
    #[serde(default)]
    pub script_execution_prices: Option<ScriptExecutionPrices>,
    #[serde(default)]
    pub max_execution_units_per_transaction: Option<ExUnits>,
    #[serde(default)]
    pub max_execution_units_per_block: Option<ExUnits>,
    #[serde(default)]
    pub max_collateral_inputs: Option<u64>,
    #[serde(default)]
    pub collateral_percentage: Option<u64>,
    #[serde(default)]
    pub max_value_size: Option<BlockSize>,
}
