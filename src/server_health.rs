//! Server health checking for Ogmios.
//!
//! This module provides functions to check the health of an Ogmios server
//! and verify it's ready to accept connections.

use crate::connection::{create_connection_object, Connection, ConnectionConfig};
use crate::error::{OgmiosError, Result};
use crate::schema::ServerHealth;
use tracing::debug;

/// Default minimum synchronization required (99.99%).
pub const DEFAULT_MIN_SYNCHRONIZATION: f64 = 0.999;

/// Get the server health.
///
/// This can be safely polled at regular intervals for monitoring.
///
/// # Arguments
///
/// * `connection` - Optional connection configuration. Uses defaults if not provided.
///
/// # Returns
///
/// The server health information.
///
/// # Example
///
/// ```rust,no_run
/// use ogmios_client::server_health::get_server_health;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let health = get_server_health(None).await?;
/// println!("Network: {:?}", health.network);
/// println!("Sync: {:.2}%", health.network_synchronization * 100.0);
/// # Ok(())
/// # }
/// ```
pub async fn get_server_health(connection: Option<ConnectionConfig>) -> Result<ServerHealth> {
    let conn = create_connection_object(connection);
    get_server_health_from_connection(&conn).await
}

/// Get the server health from a connection object.
pub async fn get_server_health_from_connection(connection: &Connection) -> Result<ServerHealth> {
    let url = format!("{}/health", connection.address.http);
    debug!("Fetching server health from {}", url);

    let response = reqwest::get(&url).await?;
    let health: ServerHealth = response.json().await?;

    Ok(health)
}

/// Options for ensuring server health.
#[derive(Debug, Clone)]
pub struct EnsureServerHealthOptions {
    /// Connection configuration.
    pub connection: Option<ConnectionConfig>,
    /// Minimum network synchronization required (0.0 to 1.0).
    pub min_synchronization: f64,
}

impl Default for EnsureServerHealthOptions {
    fn default() -> Self {
        Self {
            connection: None,
            min_synchronization: DEFAULT_MIN_SYNCHRONIZATION,
        }
    }
}

/// Ensure the server is healthy and synchronized.
///
/// This function checks that the server is ready and has sufficient network
/// synchronization to process requests reliably.
///
/// # Arguments
///
/// * `options` - Options including connection config and minimum synchronization.
///
/// # Returns
///
/// The server health information if the server is ready.
///
/// # Errors
///
/// Returns `OgmiosError::ServerNotReady` if the synchronization is below the minimum.
///
/// # Example
///
/// ```rust,no_run
/// use ogmios_client::server_health::{ensure_server_health, EnsureServerHealthOptions};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let options = EnsureServerHealthOptions {
///     min_synchronization: 0.99,
///     ..Default::default()
/// };
/// let health = ensure_server_health(options).await?;
/// println!("Server is ready at {:.2}% sync", health.network_synchronization * 100.0);
/// # Ok(())
/// # }
/// ```
pub async fn ensure_server_health(options: EnsureServerHealthOptions) -> Result<ServerHealth> {
    let health = get_server_health(options.connection).await?;

    if health.network_synchronization < options.min_synchronization {
        return Err(OgmiosError::ServerNotReady {
            synchronization: health.network_synchronization * 100.0,
            minimum: options.min_synchronization * 100.0,
        });
    }

    Ok(health)
}

/// Server not ready error with detailed information.
///
/// This is a structured error type that provides more context about why
/// the server is not ready.
#[derive(Debug, Clone)]
pub struct ServerNotReady {
    /// Current network synchronization.
    pub synchronization: f64,
    /// Required minimum synchronization.
    pub minimum: f64,
    /// Server health information.
    pub health: ServerHealth,
}

impl std::fmt::Display for ServerNotReady {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Server not ready: network synchronization is {:.2}%, minimum required is {:.2}%",
            self.synchronization * 100.0,
            self.minimum * 100.0
        )
    }
}

impl std::error::Error for ServerNotReady {}

/// Wait for the server to be ready.
///
/// This function polls the server health at regular intervals until the server
/// is synchronized enough to accept connections.
///
/// # Arguments
///
/// * `connection` - Optional connection configuration.
/// * `min_synchronization` - Minimum network synchronization required.
/// * `poll_interval` - Interval between health checks.
/// * `timeout` - Maximum time to wait for the server to be ready.
///
/// # Returns
///
/// The server health information when the server is ready.
///
/// # Errors
///
/// Returns `OgmiosError::Timeout` if the server doesn't become ready within the timeout.
pub async fn wait_for_server_ready(
    connection: Option<ConnectionConfig>,
    min_synchronization: f64,
    poll_interval: std::time::Duration,
    timeout: std::time::Duration,
) -> Result<ServerHealth> {
    let deadline = tokio::time::Instant::now() + timeout;

    loop {
        match get_server_health(connection.clone()).await {
            Ok(health) => {
                if health.network_synchronization >= min_synchronization {
                    return Ok(health);
                }
                debug!(
                    "Server sync at {:.2}%, waiting for {:.2}%",
                    health.network_synchronization * 100.0,
                    min_synchronization * 100.0
                );
            }
            Err(e) => {
                debug!("Health check failed: {}, retrying...", e);
            }
        }

        if tokio::time::Instant::now() >= deadline {
            return Err(OgmiosError::Timeout {
                timeout_ms: timeout.as_millis() as u64,
            });
        }

        tokio::time::sleep(poll_interval).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = EnsureServerHealthOptions::default();
        assert!(options.connection.is_none());
        assert_eq!(options.min_synchronization, DEFAULT_MIN_SYNCHRONIZATION);
    }
}
