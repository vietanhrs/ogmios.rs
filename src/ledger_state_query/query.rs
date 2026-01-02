//! Ledger state query functions.

use crate::connection::InteractionContext;
use crate::error::Result;
use crate::schema::{
    Address, BlockHeight, Constitution, Epoch, EraStart, EraSummary, EraWithGenesis,
    GenesisConfiguration, GovernanceProposalState, LiveStakeDistributionEntry, Point,
    ProjectedRewards, ProtocolParameters, RewardAccount, RewardAccountSummary, Slot,
    StakeAddress, StakePool, StakePoolId, StakePoolPerformance, StakePoolView, Tip,
    TransactionOutputReference, UtcTime, Utxo,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Acquire a ledger state at a specific point.
///
/// # Arguments
///
/// * `context` - The interaction context.
/// * `point` - The point to acquire the ledger state at.
///
/// # Returns
///
/// The slot number at which the ledger state was acquired.
pub async fn acquire_ledger_state(
    context: &InteractionContext,
    point: Option<Point>,
) -> Result<Slot> {
    #[derive(Serialize)]
    struct Params {
        #[serde(skip_serializing_if = "Option::is_none")]
        point: Option<Point>,
    }

    #[derive(Deserialize)]
    struct Response {
        slot: Slot,
    }

    let response: Response = context
        .request("acquireLedgerState", Some(Params { point }))
        .await?;
    Ok(response.slot)
}

/// Release the acquired ledger state.
pub async fn release_ledger_state(context: &InteractionContext) -> Result<()> {
    let _: serde_json::Value = context.request("releaseLedgerState", None::<()>).await?;
    Ok(())
}

/// Query the current constitution.
pub async fn constitution(context: &InteractionContext) -> Result<Constitution> {
    context.request("queryLedgerState/constitution", None::<()>).await
}

/// Query the current epoch.
pub async fn epoch(context: &InteractionContext) -> Result<Epoch> {
    context.request("queryLedgerState/epoch", None::<()>).await
}

/// Query the era start information.
pub async fn era_start(context: &InteractionContext) -> Result<EraStart> {
    context.request("queryLedgerState/eraStart", None::<()>).await
}

/// Query era summaries.
pub async fn era_summaries(context: &InteractionContext) -> Result<Vec<EraSummary>> {
    context.request("queryLedgerState/eraSummaries", None::<()>).await
}

/// Query genesis configuration for a specific era.
pub async fn genesis_configuration(
    context: &InteractionContext,
    era: EraWithGenesis,
) -> Result<GenesisConfiguration> {
    #[derive(Serialize)]
    struct Params {
        era: EraWithGenesis,
    }

    context
        .request("queryLedgerState/genesisConfiguration", Some(Params { era }))
        .await
}

/// Governance proposal filter.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceProposalFilter {
    /// Filter by proposal IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposals: Option<Vec<String>>,
    /// Filter by action type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
}

/// Query governance proposals.
pub async fn governance_proposals(
    context: &InteractionContext,
    filter: Option<GovernanceProposalFilter>,
) -> Result<Vec<GovernanceProposalState>> {
    context
        .request("queryLedgerState/governanceProposals", filter)
        .await
}

/// Query the ledger tip.
pub async fn ledger_tip(context: &InteractionContext) -> Result<Point> {
    context.request("queryLedgerState/tip", None::<()>).await
}

/// Query the network tip.
pub async fn network_tip(context: &InteractionContext) -> Result<Tip> {
    context.request("queryNetwork/tip", None::<()>).await
}

/// Query the network block height.
pub async fn network_block_height(context: &InteractionContext) -> Result<BlockHeight> {
    context.request("queryNetwork/blockHeight", None::<()>).await
}

/// Query live stake distribution.
pub async fn live_stake_distribution(
    context: &InteractionContext,
) -> Result<HashMap<StakePoolId, LiveStakeDistributionEntry>> {
    context
        .request("queryLedgerState/liveStakeDistribution", None::<()>)
        .await
}

/// Query the network start time.
pub async fn network_start_time(context: &InteractionContext) -> Result<UtcTime> {
    context.request("queryNetwork/startTime", None::<()>).await
}

/// Projected rewards filter.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedRewardsFilter {
    /// Stake addresses to query.
    pub stake_addresses: Vec<StakeAddress>,
}

/// Query projected rewards.
pub async fn projected_rewards(
    context: &InteractionContext,
    filter: ProjectedRewardsFilter,
) -> Result<Vec<ProjectedRewards>> {
    context
        .request("queryLedgerState/projectedRewards", Some(filter))
        .await
}

/// Query protocol parameters.
pub async fn protocol_parameters(context: &InteractionContext) -> Result<ProtocolParameters> {
    context
        .request("queryLedgerState/protocolParameters", None::<()>)
        .await
}

/// Reward account summaries filter.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardAccountSummariesFilter {
    /// Stake addresses to query.
    pub keys: Vec<StakeAddress>,
}

/// Query reward account summaries.
pub async fn reward_account_summaries(
    context: &InteractionContext,
    filter: RewardAccountSummariesFilter,
) -> Result<HashMap<RewardAccount, RewardAccountSummary>> {
    context
        .request("queryLedgerState/rewardAccountSummaries", Some(filter))
        .await
}

/// Stake pools filter.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StakePoolsFilter {
    /// Specific stake pool IDs to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake_pools: Option<Vec<StakePoolId>>,
}

/// Query stake pools.
pub async fn stake_pools(
    context: &InteractionContext,
    filter: Option<StakePoolsFilter>,
    include_stake: bool,
) -> Result<HashMap<StakePoolId, StakePoolView>> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Params {
        #[serde(skip_serializing_if = "Option::is_none")]
        stake_pools: Option<Vec<StakePoolId>>,
        include_stake: bool,
    }

    let params = Params {
        stake_pools: filter.and_then(|f| f.stake_pools),
        include_stake,
    };

    context.request("queryLedgerState/stakePools", Some(params)).await
}

/// Query stake pool performances.
pub async fn stake_pools_performances(
    context: &InteractionContext,
) -> Result<HashMap<StakePoolId, StakePoolPerformance>> {
    context
        .request("queryLedgerState/stakePoolsPerformance", None::<()>)
        .await
}

/// UTXO filter.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UtxoFilter {
    /// Filter by addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    /// Filter by output references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_references: Option<Vec<TransactionOutputReference>>,
}

/// Query UTXOs.
pub async fn utxo(
    context: &InteractionContext,
    filter: Option<UtxoFilter>,
) -> Result<Vec<Utxo>> {
    context.request("queryLedgerState/utxo", filter).await
}

/// Query UTXOs by addresses.
pub async fn utxo_by_addresses(
    context: &InteractionContext,
    addresses: Vec<Address>,
) -> Result<Vec<Utxo>> {
    utxo(
        context,
        Some(UtxoFilter {
            addresses: Some(addresses),
            output_references: None,
        }),
    )
    .await
}

/// Query UTXOs by output references.
pub async fn utxo_by_output_references(
    context: &InteractionContext,
    output_references: Vec<TransactionOutputReference>,
) -> Result<Vec<Utxo>> {
    utxo(
        context,
        Some(UtxoFilter {
            addresses: None,
            output_references: Some(output_references),
        }),
    )
    .await
}
