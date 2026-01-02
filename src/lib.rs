//! Ogmios Rust Client
//!
//! A Rust client/SDK for [Ogmios](https://ogmios.dev/), a lightweight bridge interface
//! for Cardano, providing a WebSocket-based JSON-RPC interface to interact with a
//! Cardano node.
//!
//! This crate mirrors the functionality of the official
//! [TypeScript client](https://github.com/CardanoSolutions/ogmios/tree/master/clients/TypeScript),
//! providing Rust applications with full access to Ogmios capabilities.
//!
//! # Features
//!
//! - **Chain Synchronization**: Follow the blockchain from any point, receiving
//!   notifications about new blocks and rollbacks.
//! - **Transaction Submission**: Submit and evaluate transactions.
//! - **Mempool Monitoring**: Monitor pending transactions in the mempool.
//! - **Ledger State Queries**: Query the current state of the ledger, including
//!   UTXOs, stake pools, protocol parameters, and more.
//! - **Server Health**: Check server health and synchronization status.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use ogmios_client::{
//!     connection::{ConnectionConfig, create_interaction_context, InteractionContextOptions},
//!     server_health::get_server_health,
//!     ledger_state_query::LedgerStateQueryClient,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Check server health
//!     let health = get_server_health(None).await?;
//!     println!("Connected to {} network", health.network.as_str());
//!     println!("Sync: {:.2}%", health.network_synchronization * 100.0);
//!
//!     // Create a ledger state query client
//!     let client = LedgerStateQueryClient::connect(
//!         ConnectionConfig::default(),
//!         None
//!     ).await?;
//!
//!     // Query current epoch
//!     let epoch = client.epoch().await?;
//!     println!("Current epoch: {}", epoch);
//!
//!     // Query protocol parameters
//!     let params = client.protocol_parameters().await?;
//!     println!("Min fee coefficient: {}", params.min_fee_coefficient);
//!
//!     client.shutdown().await?;
//!     Ok(())
//! }
//! ```
//!
//! # Chain Synchronization Example
//!
//! ```rust,no_run
//! use ogmios_client::{
//!     chain_synchronization::{
//!         ChainSynchronizationClient,
//!         ChainSynchronizationMessageHandlers,
//!         ChainSynchronizationClientOptions,
//!     },
//!     connection::{ConnectionConfig, create_interaction_context, InteractionContextOptions, InteractionType},
//!     schema::{Block, Point, Tip},
//!     error::Result,
//! };
//!
//! struct MyHandler;
//!
//! impl ChainSynchronizationMessageHandlers for MyHandler {
//!     fn on_roll_forward(&mut self, block: Block, tip: Tip) -> Result<()> {
//!         println!("New block at slot {} (height {})", block.slot(), block.height());
//!         Ok(())
//!     }
//!
//!     fn on_roll_backward(&mut self, point: Point, tip: Tip) -> Result<()> {
//!         println!("Rollback to {:?}", point);
//!         Ok(())
//!     }
//! }
//!
//! # async fn example() -> Result<()> {
//! let context = create_interaction_context(InteractionContextOptions {
//!     connection: ConnectionConfig::default(),
//!     interaction_type: InteractionType::LongRunning,
//!     ..Default::default()
//! }).await?;
//!
//! let client = ChainSynchronizationClient::new(
//!     context,
//!     MyHandler,
//!     ChainSynchronizationClientOptions::default()
//! ).await?;
//!
//! // Start syncing from origin
//! let intersection = client.resume(Some(vec![Point::origin()]), None).await?;
//! println!("Started at {:?}", intersection.point);
//! # Ok(())
//! # }
//! ```
//!
//! # Transaction Submission Example
//!
//! ```rust,no_run
//! use ogmios_client::{
//!     transaction_submission::TransactionSubmissionClient,
//!     connection::ConnectionConfig,
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = TransactionSubmissionClient::connect(ConnectionConfig::default()).await?;
//!
//! // Evaluate transaction costs
//! let tx_cbor = "84a400..."; // Your transaction CBOR
//! let costs = client.evaluate_transaction(tx_cbor, None).await?;
//! for cost in &costs {
//!     println!("Script {:?}: {} mem, {} cpu",
//!         cost.validator, cost.budget.memory, cost.budget.cpu);
//! }
//!
//! // Submit the transaction
//! let tx_id = client.submit_transaction(tx_cbor).await?;
//! println!("Submitted: {}", tx_id);
//!
//! client.shutdown().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Module Structure
//!
//! - [`schema`]: All Cardano type definitions (blocks, transactions, etc.)
//! - [`connection`]: Connection management and WebSocket handling
//! - [`server_health`]: Server health checking
//! - [`chain_synchronization`]: Chain sync client for following the blockchain
//! - [`transaction_submission`]: Transaction submission and evaluation
//! - [`mempool_monitoring`]: Mempool monitoring client
//! - [`ledger_state_query`]: Ledger state queries
//! - [`util`]: Utility functions
//! - [`error`]: Error types

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod chain_synchronization;
pub mod connection;
pub mod error;
pub mod ledger_state_query;
pub mod mempool_monitoring;
// TODO: Add documentation for the schema module
#[allow(missing_docs)]
pub mod schema;
pub mod server_health;
pub mod transaction_submission;
pub mod util;

// Re-export main types at crate root for convenience
pub use chain_synchronization::{
    create_chain_synchronization_client, ChainSynchronizationClient,
    ChainSynchronizationClientOptions, ChainSynchronizationMessageHandlers, Intersection,
};

pub use connection::{
    create_connection_object, create_interaction_context, Connection, ConnectionConfig,
    InteractionContext, InteractionContextOptions, InteractionType,
};

pub use error::{OgmiosError, Result};

pub use ledger_state_query::{
    create_ledger_state_query_client, LedgerStateQueryClient, LedgerStateQueryClientOptions,
};

pub use mempool_monitoring::{create_mempool_monitoring_client, MempoolMonitoringClient};

pub use server_health::{
    ensure_server_health, get_server_health, wait_for_server_ready, EnsureServerHealthOptions,
};

pub use transaction_submission::{
    create_transaction_submission_client, TransactionSubmissionClient,
};

// Re-export commonly used schema types
pub use schema::{
    // Primitives
    Address,
    Assets,
    // Blocks
    Block,
    BlockBFT,
    BlockEBB,
    BlockHeight,
    BlockPraos,
    // Scripts
    Datum,
    Epoch,
    // Era
    Era,
    EraSummary,
    EraWithGenesis,
    Lovelace,
    // Network
    Network,
    Point,
    PolicyId,
    // Protocol
    ProtocolParameters,
    Script,
    ServerHealth,
    Slot,
    StakeAddress,
    StakePoolId,
    Tip,
    // Transactions
    Transaction,
    TransactionId,
    TransactionInput,
    TransactionOutput,
    TransactionOutputReference,
    Utxo,
    Value,
};

/// Prelude module for convenient imports.
///
/// ```rust
/// use ogmios_client::prelude::*;
/// ```
pub mod prelude {
    pub use crate::chain_synchronization::{
        create_chain_synchronization_client, ChainSynchronizationClient,
        ChainSynchronizationClientOptions, ChainSynchronizationMessageHandlers,
    };
    pub use crate::connection::{
        create_connection_object, create_interaction_context, Connection, ConnectionConfig,
        InteractionContext, InteractionContextOptions, InteractionType,
    };
    pub use crate::error::{OgmiosError, Result};
    pub use crate::ledger_state_query::{create_ledger_state_query_client, LedgerStateQueryClient};
    pub use crate::mempool_monitoring::{
        create_mempool_monitoring_client, MempoolMonitoringClient,
    };
    pub use crate::schema::{
        Address, Block, BlockHeight, Epoch, Lovelace, Point, Slot, Tip, Transaction, TransactionId,
        Value,
    };
    pub use crate::server_health::{ensure_server_health, get_server_health};
    pub use crate::transaction_submission::{
        create_transaction_submission_client, TransactionSubmissionClient,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prelude_imports() {
        #[allow(unused_imports)]
        use crate::prelude::*;
        // Just verify the prelude compiles correctly
    }

    #[test]
    fn test_point_creation() {
        let origin = Point::origin();
        assert!(matches!(origin, Point::Origin(_)));

        let point = Point::at(12345, "abcdef");
        assert!(matches!(point, Point::Point { .. }));
    }

    #[test]
    fn test_value_creation() {
        let value = Value::ada_only(1_000_000);
        assert_eq!(value.lovelace(), 1_000_000);
    }
}
