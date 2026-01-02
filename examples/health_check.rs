//! Example: Health Check
//!
//! This example demonstrates how to set up a connection configuration
//! and perform a health check on an Ogmios server.
//!
//! Run with: cargo run --example health_check
//!
//! You can also specify a custom host and port:
//!   OGMIOS_HOST=localhost OGMIOS_PORT=1337 cargo run --example health_check

use ogmios_client::{
    connection::ConnectionConfig,
    schema::Tip,
    server_health::{get_server_health, ensure_server_health, EnsureServerHealthOptions},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read connection settings from environment variables or use defaults
    let host = std::env::var("OGMIOS_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = std::env::var("OGMIOS_PORT")
        .unwrap_or_else(|_| "1337".to_string())
        .parse()
        .expect("OGMIOS_PORT must be a valid port number");
    let tls = std::env::var("OGMIOS_TLS")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    println!("Ogmios Health Check Example");
    println!("============================");
    println!("Connecting to: {}://{}:{}", if tls { "https" } else { "http" }, host, port);
    println!();

    // Create a connection configuration
    let connection = ConnectionConfig {
        host: host.clone(),
        port,
        tls,
        max_payload: 65536, // 64KB default
    };

    // Method 1: Simple health check
    println!("1. Simple Health Check");
    println!("-----------------------");

    match get_server_health(Some(connection.clone())).await {
        Ok(health) => {
            println!("Server is healthy!");
            println!("  Start time: {}", health.start_time);
            println!("  Last known tip:");
            match &health.last_known_tip {
                Tip::Origin(_) => {
                    println!("    Chain is at origin (empty)");
                }
                Tip::Tip { slot, id, height } => {
                    println!("    Slot: {}", slot);
                    println!("    Height: {}", height);
                    println!("    ID: {}", id);
                }
            }
            if let Some(last_update) = &health.last_tip_update {
                println!("  Last tip update: {}", last_update);
            }
            println!("  Network sync: {:.2}%", health.network_synchronization * 100.0);
            println!("  Current era: {:?}", health.current_era);
            println!("  Version: {}", health.version);
            println!();
        }
        Err(e) => {
            eprintln!("Health check failed: {}", e);
            eprintln!("Make sure Ogmios is running at {}:{}", host, port);
            return Err(e.into());
        }
    }

    // Method 2: Ensure server health with synchronization threshold
    println!("2. Ensure Server Health (with sync threshold)");
    println!("----------------------------------------------");

    let options = EnsureServerHealthOptions {
        connection: Some(ConnectionConfig {
            host: host.clone(),
            port,
            tls,
            max_payload: 65536,
        }),
        min_synchronization: 0.90, // Require at least 90% sync
    };

    match ensure_server_health(options).await {
        Ok(health) => {
            println!("Server meets synchronization requirements!");
            println!("  Network sync: {:.2}%", health.network_synchronization * 100.0);
            println!();
        }
        Err(e) => {
            eprintln!("Server health check failed: {}", e);
            eprintln!("The server may not be sufficiently synchronized.");
            return Err(e.into());
        }
    }

    println!("All health checks passed!");
    Ok(())
}
