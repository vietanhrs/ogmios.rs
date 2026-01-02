//! Network types for Cardano.

use serde::{Deserialize, Serialize};
use super::primitives::*;
use super::era::Era;

/// Cardano network names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Network {
    Mainnet,
    Preview,
    Preprod,
    #[serde(other)]
    Other,
}

impl Network {
    /// Get the network as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Network::Mainnet => "mainnet",
            Network::Preview => "preview",
            Network::Preprod => "preprod",
            Network::Other => "unknown",
        }
    }
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Server health information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerHealth {
    /// Current era.
    pub current_era: Era,
    /// Last known tip.
    pub last_known_tip: Tip,
    /// Last tip update time.
    #[serde(default)]
    pub last_tip_update: Option<UtcTime>,
    /// Server metrics.
    pub metrics: ServerMetrics,
    /// Server start time.
    pub start_time: UtcTime,
    /// Network name.
    pub network: Network,
    /// Network synchronization percentage (0.0 to 1.0).
    pub network_synchronization: f64,
    /// Server version.
    pub version: String,
}

/// Server metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMetrics {
    /// Runtime statistics (optional).
    #[serde(default)]
    pub runtime_stats: Option<RuntimeStats>,
    /// Session durations.
    pub session_durations: SessionDurations,
    /// Total connections since start.
    pub total_connections: u64,
    /// Total messages processed.
    pub total_messages: u64,
    /// Total unrouted messages.
    pub total_unrouted: u64,
    /// Currently active connections.
    pub active_connections: u64,
}

/// Runtime statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeStats {
    /// GC CPU time in seconds.
    #[serde(default)]
    pub gc_cpu_time: Option<f64>,
    /// CPU time in seconds.
    #[serde(default)]
    pub cpu_time: Option<f64>,
    /// Maximum heap size in bytes.
    #[serde(default)]
    pub max_heap_size: Option<u64>,
    /// Current heap size in bytes.
    #[serde(default)]
    pub current_heap_size: Option<u64>,
}

/// Session duration statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDurations {
    /// Maximum session duration.
    pub max: f64,
    /// Mean session duration.
    pub mean: f64,
    /// Minimum session duration.
    pub min: f64,
}

/// Mempool size and capacity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MempoolSizeAndCapacity {
    /// Current number of bytes in the mempool.
    pub bytes: NumberOfBytes,
    /// Current number of transactions in the mempool.
    pub transactions: u64,
    /// Maximum capacity in bytes.
    pub max_bytes: NumberOfBytes,
    /// Maximum number of transactions.
    pub max_transactions: u64,
}

/// Reward account summaries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardAccountSummary {
    /// Reward account address.
    pub address: RewardAccount,
    /// Delegated stake pool (if any).
    #[serde(default)]
    pub delegate: Option<StakePoolId>,
    /// Current rewards balance.
    pub rewards: AdaValue,
    /// Deposit amount.
    pub deposit: AdaValue,
}

/// Live stake distribution entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveStakeDistributionEntry {
    /// Stake pool ID.
    pub stake_pool: StakePoolId,
    /// Total stake delegated.
    pub stake: AdaValue,
}

/// Projected rewards.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedRewards {
    /// Stake address.
    pub address: StakeAddress,
    /// Projected rewards.
    pub rewards: AdaValue,
}
