//! Example: Chain Synchronization
//!
//! This example demonstrates how to use the ChainSynchronizationClient to follow
//! the Cardano blockchain. It shows how to:
//! - Connect to Ogmios and start syncing from a specific point
//! - Handle new blocks (roll forward events)
//! - Handle rollbacks (roll backward events)
//! - Gracefully shutdown the client
//!
//! Run with: cargo run --example chain_sync
//!
//! You can specify custom connection settings:
//!   OGMIOS_HOST=localhost OGMIOS_PORT=1337 cargo run --example chain_sync
//!
//! You can also limit the number of blocks to process:
//!   MAX_BLOCKS=10 cargo run --example chain_sync

use ogmios_client::{
    chain_synchronization::{
        ChainSynchronizationClient, ChainSynchronizationClientOptions,
        ChainSynchronizationMessageHandlers,
    },
    connection::{
        create_interaction_context, ConnectionConfig, InteractionContextOptions, InteractionType,
    },
    error::Result,
    schema::{Block, Point, Tip},
    server_health::get_server_health,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// A custom handler that processes blocks and tracks statistics.
struct BlockHandler {
    /// Counter for processed blocks.
    block_count: Arc<AtomicUsize>,
    /// Counter for rollbacks.
    rollback_count: Arc<AtomicUsize>,
    /// Maximum number of blocks to process (None for unlimited).
    max_blocks: Option<usize>,
}

impl BlockHandler {
    fn new(max_blocks: Option<usize>) -> Self {
        Self {
            block_count: Arc::new(AtomicUsize::new(0)),
            rollback_count: Arc::new(AtomicUsize::new(0)),
            max_blocks,
        }
    }

    fn block_count(&self) -> usize {
        self.block_count.load(Ordering::SeqCst)
    }

    fn rollback_count(&self) -> usize {
        self.rollback_count.load(Ordering::SeqCst)
    }

    fn should_continue(&self) -> bool {
        self.max_blocks
            .map_or(true, |max| self.block_count() < max)
    }
}

impl ChainSynchronizationMessageHandlers for BlockHandler {
    fn on_roll_forward(&mut self, block: Block, tip: Tip) -> Result<()> {
        let count = self.block_count.fetch_add(1, Ordering::SeqCst) + 1;

        println!("\n=== Block #{} ===", count);
        println!("  Slot: {}", block.slot());
        println!("  Height: {}", block.height());
        println!("  Hash: {}", block.id());

        // Display block type and era
        let (block_type, era, tx_count) = match &block {
            Block::EBB(b) => (&b.block_type, &b.era, 0),
            Block::BFT(b) => (&b.block_type, &b.era, b.transactions.len()),
            Block::Praos(b) => (&b.block_type, &b.era, b.transactions.len()),
        };
        println!("  Type: {}", block_type);
        println!("  Era: {}", era);
        println!("  Transactions: {}", tx_count);

        // Display tip information
        match &tip {
            Tip::Origin(_) => {
                println!("  Tip: Origin");
            }
            Tip::Tip { slot, height, .. } => {
                println!("  Tip Slot: {}", slot);
                println!("  Tip Height: {}", height);
            }
        }

        // Check if we should stop
        if let Some(max) = self.max_blocks {
            if count >= max {
                println!("\nReached maximum block count ({}), stopping...", max);
                return Err(ogmios_client::error::OgmiosError::ConnectionClosed);
            }
        }

        Ok(())
    }

    fn on_roll_backward(&mut self, point: Point, tip: Tip) -> Result<()> {
        let count = self.rollback_count.fetch_add(1, Ordering::SeqCst) + 1;

        println!("\n!!! Rollback #{} !!!", count);
        match &point {
            Point::Origin(_) => {
                println!("  Rolled back to: Origin");
            }
            Point::Point { slot, id } => {
                println!("  Rolled back to slot: {}", slot);
                println!("  Point ID: {}", id);
            }
        }

        match &tip {
            Tip::Origin(_) => {
                println!("  New tip: Origin");
            }
            Tip::Tip { slot, height, .. } => {
                println!("  New tip slot: {}", slot);
                println!("  New tip height: {}", height);
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Ogmios Chain Synchronization Example");
    println!("====================================\n");

    // Read connection settings from environment
    let host = std::env::var("OGMIOS_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = std::env::var("OGMIOS_PORT")
        .unwrap_or_else(|_| "1337".to_string())
        .parse()
        .expect("OGMIOS_PORT must be a valid port number");
    let tls = std::env::var("OGMIOS_TLS")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    // Optional: limit number of blocks to process
    let max_blocks = std::env::var("MAX_BLOCKS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok());

    if let Some(max) = max_blocks {
        println!("Will process a maximum of {} blocks\n", max);
    }

    let connection = ConnectionConfig {
        host: host.clone(),
        port,
        tls,
        max_payload: 65536,
    };

    println!("Connection: {}://{}:{}", if tls { "wss" } else { "ws" }, host, port);

    // First, check server health
    println!("\nChecking server health...");
    let health = get_server_health(Some(connection.clone())).await?;
    println!("  Network: {}", health.network.as_str());
    println!("  Sync: {:.2}%", health.network_synchronization * 100.0);
    println!("  Current era: {:?}", health.current_era);

    match &health.last_known_tip {
        Tip::Origin(_) => {
            println!("  Chain tip: Origin (empty chain)");
        }
        Tip::Tip { slot, height, .. } => {
            println!("  Chain tip: Slot {}, Height {}", slot, height);
        }
    }

    // Create the chain synchronization client
    println!("\nCreating chain synchronization client...");

    let context = create_interaction_context(InteractionContextOptions {
        connection,
        interaction_type: InteractionType::LongRunning,
        ..Default::default()
    })
    .await?;

    let handler = BlockHandler::new(max_blocks);
    let client = ChainSynchronizationClient::new(
        context,
        handler,
        ChainSynchronizationClientOptions::default(),
    )
    .await?;

    // Start syncing from origin
    // You can also specify specific points to start from:
    // let points = vec![Point::at(12345, "block_hash_here")];
    println!("\nStarting chain synchronization from origin...");
    let intersection = client.resume(Some(vec![Point::origin()]), None).await?;

    println!("Sync started!");
    match &intersection.point {
        Point::Origin(_) => {
            println!("  Intersection: Origin");
        }
        Point::Point { slot, id } => {
            println!("  Intersection slot: {}", slot);
            println!("  Intersection ID: {}", id);
        }
    }

    match &intersection.tip {
        Tip::Origin(_) => {
            println!("  Tip: Origin");
        }
        Tip::Tip { slot, height, .. } => {
            println!("  Tip: Slot {}, Height {}", slot, height);
        }
    }

    println!("\nProcessing blocks... (Press Ctrl+C to stop)\n");

    // Set up graceful shutdown
    let client_clone = Arc::new(client);
    let client_for_shutdown = client_clone.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
        println!("\n\nShutting down gracefully...");
        if let Err(e) = client_for_shutdown.shutdown().await {
            eprintln!("Error during shutdown: {}", e);
        }
    });

    // Wait for the client to stop (either by max blocks or Ctrl+C)
    while client_clone.is_running() {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    println!("\n=== Summary ===");
    println!("Chain synchronization stopped.");

    Ok(())
}
