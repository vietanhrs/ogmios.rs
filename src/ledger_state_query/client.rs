//! Ledger State Query client implementation.

use crate::connection::{
    create_interaction_context, ConnectionConfig, InteractionContext, InteractionContextOptions,
    InteractionType,
};
use crate::error::Result;
use crate::schema::{
    Address, BlockHeight, Epoch, EraStart, EraSummary, EraWithGenesis,
    Constitution, GenesisConfiguration, GovernanceProposalState,
    LiveStakeDistributionEntry, Point, ProjectedRewards, ProtocolParameters,
    RewardAccount, RewardAccountSummary, Slot, StakeAddress, StakePoolId,
    StakePoolPerformance, StakePoolView, Tip, TransactionOutputReference, UtcTime, Utxo,
};
use std::collections::HashMap;
use std::sync::Arc;

use super::query::{self, *};

/// Options for creating a ledger state query client.
#[derive(Debug, Clone, Default)]
pub struct LedgerStateQueryClientOptions {
    /// Automatically acquire ledger state at this point.
    pub point: Option<Point>,
}

/// A ledger state query client for querying blockchain state.
///
/// This client provides methods for querying various aspects of the
/// Cardano ledger state, such as UTXOs, stake pools, protocol parameters,
/// and governance proposals.
///
/// # Example
///
/// ```rust,no_run
/// use ogmios_client::ledger_state_query::LedgerStateQueryClient;
/// use ogmios_client::connection::ConnectionConfig;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = LedgerStateQueryClient::connect(
///     ConnectionConfig::default(),
///     None
/// ).await?;
///
/// // Get current epoch
/// let epoch = client.epoch().await?;
/// println!("Current epoch: {}", epoch);
///
/// // Get protocol parameters
/// let params = client.protocol_parameters().await?;
/// println!("Min fee coefficient: {}", params.min_fee_coefficient);
///
/// // Query UTXOs for an address
/// let utxos = client.utxo_by_addresses(vec!["addr_test1...".to_string()]).await?;
/// println!("Found {} UTXOs", utxos.len());
///
/// client.shutdown().await?;
/// # Ok(())
/// # }
/// ```
pub struct LedgerStateQueryClient {
    /// The interaction context.
    context: Arc<InteractionContext>,
}

impl LedgerStateQueryClient {
    /// Create a new ledger state query client from an existing context.
    pub fn new(context: InteractionContext) -> Self {
        Self {
            context: Arc::new(context),
        }
    }

    /// Connect to Ogmios and create a new ledger state query client.
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection configuration.
    /// * `options` - Optional client options.
    pub async fn connect(
        connection: ConnectionConfig,
        options: Option<LedgerStateQueryClientOptions>,
    ) -> Result<Self> {
        let context = create_interaction_context(InteractionContextOptions {
            connection,
            interaction_type: InteractionType::LongRunning,
            ..Default::default()
        })
        .await?;

        let client = Self::new(context);

        // Optionally acquire ledger state at a specific point
        if let Some(opts) = options {
            if let Some(point) = opts.point {
                client.acquire_ledger_state(Some(point)).await?;
            }
        }

        Ok(client)
    }

    /// Get a reference to the interaction context.
    pub fn context(&self) -> &InteractionContext {
        &self.context
    }

    /// Acquire a ledger state at a specific point.
    pub async fn acquire_ledger_state(&self, point: Option<Point>) -> Result<Slot> {
        query::acquire_ledger_state(&self.context, point).await
    }

    /// Release the acquired ledger state.
    pub async fn release_ledger_state(&self) -> Result<()> {
        query::release_ledger_state(&self.context).await
    }

    /// Query the current constitution.
    pub async fn constitution(&self) -> Result<Constitution> {
        query::constitution(&self.context).await
    }

    /// Query the current epoch.
    pub async fn epoch(&self) -> Result<Epoch> {
        query::epoch(&self.context).await
    }

    /// Query the era start information.
    pub async fn era_start(&self) -> Result<EraStart> {
        query::era_start(&self.context).await
    }

    /// Query era summaries.
    pub async fn era_summaries(&self) -> Result<Vec<EraSummary>> {
        query::era_summaries(&self.context).await
    }

    /// Query genesis configuration for a specific era.
    pub async fn genesis_configuration(&self, era: EraWithGenesis) -> Result<GenesisConfiguration> {
        query::genesis_configuration(&self.context, era).await
    }

    /// Query governance proposals.
    pub async fn governance_proposals(
        &self,
        filter: Option<GovernanceProposalFilter>,
    ) -> Result<Vec<GovernanceProposalState>> {
        query::governance_proposals(&self.context, filter).await
    }

    /// Query the ledger tip.
    pub async fn ledger_tip(&self) -> Result<Point> {
        query::ledger_tip(&self.context).await
    }

    /// Query the network tip.
    pub async fn network_tip(&self) -> Result<Tip> {
        query::network_tip(&self.context).await
    }

    /// Query the network block height.
    pub async fn network_block_height(&self) -> Result<BlockHeight> {
        query::network_block_height(&self.context).await
    }

    /// Query live stake distribution.
    pub async fn live_stake_distribution(
        &self,
    ) -> Result<HashMap<StakePoolId, LiveStakeDistributionEntry>> {
        query::live_stake_distribution(&self.context).await
    }

    /// Query the network start time.
    pub async fn network_start_time(&self) -> Result<UtcTime> {
        query::network_start_time(&self.context).await
    }

    /// Query projected rewards.
    pub async fn projected_rewards(
        &self,
        stake_addresses: Vec<StakeAddress>,
    ) -> Result<Vec<ProjectedRewards>> {
        query::projected_rewards(
            &self.context,
            ProjectedRewardsFilter { stake_addresses },
        )
        .await
    }

    /// Query protocol parameters.
    pub async fn protocol_parameters(&self) -> Result<ProtocolParameters> {
        query::protocol_parameters(&self.context).await
    }

    /// Query reward account summaries.
    pub async fn reward_account_summaries(
        &self,
        keys: Vec<StakeAddress>,
    ) -> Result<HashMap<RewardAccount, RewardAccountSummary>> {
        query::reward_account_summaries(&self.context, RewardAccountSummariesFilter { keys }).await
    }

    /// Query stake pools.
    pub async fn stake_pools(
        &self,
        filter: Option<StakePoolsFilter>,
        include_stake: bool,
    ) -> Result<HashMap<StakePoolId, StakePoolView>> {
        query::stake_pools(&self.context, filter, include_stake).await
    }

    /// Query stake pool performances.
    pub async fn stake_pools_performances(
        &self,
    ) -> Result<HashMap<StakePoolId, StakePoolPerformance>> {
        query::stake_pools_performances(&self.context).await
    }

    /// Query UTXOs.
    pub async fn utxo(&self, filter: Option<UtxoFilter>) -> Result<Vec<Utxo>> {
        query::utxo(&self.context, filter).await
    }

    /// Query UTXOs by addresses.
    pub async fn utxo_by_addresses(&self, addresses: Vec<Address>) -> Result<Vec<Utxo>> {
        query::utxo_by_addresses(&self.context, addresses).await
    }

    /// Query UTXOs by output references.
    pub async fn utxo_by_output_references(
        &self,
        output_references: Vec<TransactionOutputReference>,
    ) -> Result<Vec<Utxo>> {
        query::utxo_by_output_references(&self.context, output_references).await
    }

    /// Shutdown the client.
    pub async fn shutdown(&self) -> Result<()> {
        self.context.shutdown().await
    }
}

/// Create a ledger state query client.
///
/// This is a convenience function that creates a connection and client in one step.
pub async fn create_ledger_state_query_client(
    connection: ConnectionConfig,
    options: Option<LedgerStateQueryClientOptions>,
) -> Result<LedgerStateQueryClient> {
    LedgerStateQueryClient::connect(connection, options).await
}
