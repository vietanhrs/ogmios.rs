//! Genesis configuration types for different Cardano eras.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::primitives::*;
use super::protocol::*;

/// Genesis configuration - varies by era.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "era", rename_all = "camelCase")]
pub enum GenesisConfiguration {
    /// Byron genesis configuration.
    Byron(GenesisByron),
    /// Shelley genesis configuration.
    Shelley(GenesisShelley),
    /// Alonzo genesis configuration.
    Alonzo(GenesisAlonzo),
    /// Conway genesis configuration.
    Conway(GenesisConway),
}

/// Byron genesis configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisByron {
    /// Genesis key hashes.
    #[serde(default)]
    pub genesis_key_hashes: Vec<DigestBlake2b224>,
    /// Genesis delegates.
    #[serde(default)]
    pub genesis_delegates: HashMap<String, GenesisDelegate>,
    /// Start time.
    pub start_time: UtcTime,
    /// Initial funds.
    #[serde(default)]
    pub initial_funds: HashMap<Address, Lovelace>,
    /// Security parameter.
    pub security_parameter: u64,
    /// Network magic.
    pub network_magic: NetworkMagic,
    /// Protocol parameters.
    #[serde(default)]
    pub protocol_parameters: Option<BootstrapProtocolParameters>,
}

/// Genesis delegate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisDelegate {
    /// Delegate key hash.
    pub delegate: DigestBlake2b224,
    /// VRF key hash.
    pub vrf: VrfVerificationKey,
}

/// Bootstrap (Byron) protocol parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapProtocolParameters {
    /// Minimum fee coefficient.
    pub min_fee_coefficient: u64,
    /// Minimum fee constant.
    pub min_fee_constant: Lovelace,
    /// Maximum block size.
    pub max_block_body_size: NumberOfBytes,
    /// Maximum header size.
    pub max_block_header_size: NumberOfBytes,
    /// Maximum transaction size.
    pub max_transaction_size: NumberOfBytes,
    /// Max proposals per epoch.
    #[serde(default)]
    pub max_update_proposals_per_epoch: Option<u64>,
}

/// Shelley genesis configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisShelley {
    /// Network ID.
    pub network: String,
    /// Network magic.
    pub network_magic: NetworkMagic,
    /// Start time.
    pub start_time: UtcTime,
    /// Security parameter.
    pub security_parameter: u64,
    /// Active slots coefficient.
    pub active_slots_coefficient: Ratio,
    /// Epoch length.
    pub epoch_length: u64,
    /// Slots per KES period.
    pub slots_per_kes_period: u64,
    /// Max KES evolutions.
    pub max_kes_evolutions: u64,
    /// Slot length in seconds.
    pub slot_length: RelativeTime,
    /// Update quorum.
    pub update_quorum: u64,
    /// Max lovelace supply.
    pub max_lovelace_supply: Lovelace,
    /// Initial funds.
    #[serde(default)]
    pub initial_funds: HashMap<Address, Lovelace>,
    /// Initial stake pools.
    #[serde(default)]
    pub initial_stake_pools: GenesisStakePools,
    /// Initial delegates.
    #[serde(default)]
    pub initial_delegates: Vec<InitialDelegate>,
    /// Protocol parameters.
    #[serde(default)]
    pub protocol_parameters: Option<ProtocolParameters>,
}

/// Genesis stake pools configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenesisStakePools {
    /// Stake pool registrations.
    #[serde(default)]
    pub stake_pools: HashMap<StakePoolId, serde_json::Value>,
    /// Stake delegations.
    #[serde(default)]
    pub delegators: HashMap<DigestBlake2b224, StakePoolId>,
}

/// Initial delegate configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialDelegate {
    /// Issuer (genesis key hash).
    pub issuer: DigestBlake2b224,
    /// Delegate.
    pub delegate: DigestBlake2b224,
    /// VRF key.
    pub vrf: VrfVerificationKey,
}

/// Alonzo genesis configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisAlonzo {
    /// Plutus cost models.
    pub cost_models: CostModels,
    /// Script execution prices.
    pub prices: ScriptExecutionPrices,
    /// Max execution units per transaction.
    pub max_execution_units_per_transaction: super::transaction::ExUnits,
    /// Max execution units per block.
    pub max_execution_units_per_block: super::transaction::ExUnits,
    /// Max value size.
    pub max_value_size: NumberOfBytes,
    /// Collateral percentage.
    pub collateral_percentage: u64,
    /// Max collateral inputs.
    pub max_collateral_inputs: u64,
}

/// Conway genesis configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisConway {
    /// Constitution.
    #[serde(default)]
    pub constitution: Option<super::governance::Constitution>,
    /// Constitutional committee.
    #[serde(default)]
    pub constitutional_committee: Option<ConstitutionalCommitteeConfig>,
    /// DRep voting thresholds.
    #[serde(default)]
    pub delegate_representative_voting_thresholds: Option<DelegateRepresentativeVotingThresholds>,
    /// Stake pool voting thresholds.
    #[serde(default)]
    pub stake_pool_voting_thresholds: Option<StakePoolVotingThresholds>,
    /// Governance action lifetime.
    #[serde(default)]
    pub governance_action_lifetime: Option<u64>,
    /// Governance action deposit.
    #[serde(default)]
    pub governance_action_deposit: Option<AdaValue>,
    /// DRep deposit.
    #[serde(default)]
    pub delegate_representative_deposit: Option<AdaValue>,
    /// DRep max idle time.
    #[serde(default)]
    pub delegate_representative_max_idle_time: Option<u64>,
    /// Constitutional committee min size.
    #[serde(default)]
    pub constitutional_committee_min_size: Option<u64>,
    /// Constitutional committee max term length.
    #[serde(default)]
    pub constitutional_committee_max_term_length: Option<u64>,
    /// Plutus V3 cost model.
    #[serde(default)]
    pub plutus_cost_models: Option<CostModels>,
    /// Min fee for reference scripts.
    #[serde(default)]
    pub min_fee_reference_scripts: Option<MinFeeReferenceScripts>,
}

/// Constitutional committee initial configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstitutionalCommitteeConfig {
    /// Initial members.
    #[serde(default)]
    pub members: Vec<super::governance::ConstitutionalCommitteeMember>,
    /// Quorum threshold.
    pub quorum: Ratio,
}
