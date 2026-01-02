# ogmios-client

A Rust client/SDK for [Ogmios](https://ogmios.dev/), a lightweight bridge interface for Cardano.

This crate mirrors the functionality of the official [TypeScript client](https://github.com/CardanoSolutions/ogmios/tree/master/clients/TypeScript), providing Rust applications with full access to Ogmios capabilities.

## Features

- **Chain Synchronization**: Follow the blockchain from any point, receiving notifications about new blocks and rollbacks
- **Transaction Submission**: Submit and evaluate transactions
- **Mempool Monitoring**: Monitor pending transactions in the mempool
- **Ledger State Queries**: Query the current state of the ledger (UTXOs, stake pools, protocol parameters, governance proposals, etc.)
- **Server Health**: Check server health and synchronization status

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ogmios-client = "0.1"
tokio = { version = "1.35", features = ["full"] }
```

## Quick Start

```rust
use ogmios_client::{
    connection::ConnectionConfig,
    server_health::get_server_health,
    ledger_state_query::LedgerStateQueryClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check server health
    let health = get_server_health(None).await?;
    println!("Connected to {} network", health.network.as_str());
    println!("Sync: {:.2}%", health.network_synchronization * 100.0);

    // Create a ledger state query client
    let client = LedgerStateQueryClient::connect(
        ConnectionConfig::default(),
        None
    ).await?;

    // Query current epoch
    let epoch = client.epoch().await?;
    println!("Current epoch: {}", epoch);

    // Query protocol parameters
    let params = client.protocol_parameters().await?;
    println!("Min fee coefficient: {}", params.min_fee_coefficient);

    client.shutdown().await?;
    Ok(())
}
```

## Chain Synchronization

```rust
use ogmios_client::{
    chain_synchronization::{
        ChainSynchronizationClient,
        ChainSynchronizationMessageHandlers,
        ChainSynchronizationClientOptions,
    },
    connection::{ConnectionConfig, create_interaction_context, InteractionContextOptions, InteractionType},
    schema::{Block, Point, Tip},
    error::Result,
};

struct MyHandler;

impl ChainSynchronizationMessageHandlers for MyHandler {
    fn on_roll_forward(&mut self, block: Block, tip: Tip) -> Result<()> {
        println!("New block at slot {} (height {})", block.slot(), block.height());
        Ok(())
    }

    fn on_roll_backward(&mut self, point: Point, tip: Tip) -> Result<()> {
        println!("Rollback to {:?}", point);
        Ok(())
    }
}

async fn sync_from_origin() -> Result<()> {
    let context = create_interaction_context(InteractionContextOptions {
        connection: ConnectionConfig::default(),
        interaction_type: InteractionType::LongRunning,
        ..Default::default()
    }).await?;

    let client = ChainSynchronizationClient::new(
        context,
        MyHandler,
        ChainSynchronizationClientOptions::default()
    ).await?;

    // Start syncing from origin
    let intersection = client.resume(Some(vec![Point::origin()]), None).await?;
    println!("Started at {:?}", intersection.point);
    Ok(())
}
```

## Transaction Submission

```rust
use ogmios_client::{
    transaction_submission::TransactionSubmissionClient,
    connection::ConnectionConfig,
};

async fn submit_tx() -> Result<(), Box<dyn std::error::Error>> {
    let client = TransactionSubmissionClient::connect(ConnectionConfig::default()).await?;

    // Evaluate transaction costs
    let tx_cbor = "84a400..."; // Your transaction CBOR
    let costs = client.evaluate_transaction(tx_cbor, None).await?;
    for cost in &costs {
        println!("Script {:?}: {} mem, {} cpu",
            cost.validator, cost.budget.memory, cost.budget.cpu);
    }

    // Submit the transaction
    let tx_id = client.submit_transaction(tx_cbor).await?;
    println!("Submitted: {}", tx_id);

    client.shutdown().await?;
    Ok(())
}
```

## Mempool Monitoring

```rust
use ogmios_client::{
    mempool_monitoring::MempoolMonitoringClient,
    connection::ConnectionConfig,
};

async fn monitor_mempool() -> Result<(), Box<dyn std::error::Error>> {
    let client = MempoolMonitoringClient::connect(ConnectionConfig::default()).await?;

    // Acquire mempool snapshot
    let slot = client.acquire_mempool().await?;
    println!("Acquired mempool at slot {}", slot);

    // Get mempool size
    let size = client.size_of_mempool().await?;
    println!("Mempool has {} transactions ({} bytes)", size.transactions, size.bytes);

    // Iterate through transactions
    while let Some(tx) = client.next_transaction().await? {
        println!("Transaction: {}", tx.id);
    }

    client.release_mempool().await?;
    client.shutdown().await?;
    Ok(())
}
```

## Module Structure

- `schema` - All Cardano type definitions (blocks, transactions, governance, etc.)
- `connection` - Connection management and WebSocket handling
- `server_health` - Server health checking
- `chain_synchronization` - Chain sync client for following the blockchain
- `transaction_submission` - Transaction submission and evaluation
- `mempool_monitoring` - Mempool monitoring client
- `ledger_state_query` - Ledger state queries
- `util` - Utility functions
- `error` - Error types

## Connection Configuration

```rust
use ogmios_client::connection::ConnectionConfig;

// Default configuration (localhost:1337)
let config = ConnectionConfig::default();

// Custom configuration
let config = ConnectionConfig::new("my-ogmios-server.com", 1337)
    .with_tls()  // Use wss://
    .with_max_payload(256 * 1024 * 1024);  // 256MB max payload
```

## Requirements

- Rust 1.70+
- A running Ogmios server (v6.x)

## License

MIT
