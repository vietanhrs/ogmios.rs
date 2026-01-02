//! Mempool Monitoring client implementation.

use crate::connection::{
    create_interaction_context, ConnectionConfig, InteractionContext, InteractionContextOptions,
    InteractionType,
};
use crate::error::Result;
use crate::schema::{MempoolSizeAndCapacity, Slot, Transaction, TransactionId};
use std::sync::Arc;

use super::{
    acquire_mempool, has_transaction, next_transaction, next_transaction_id, release_mempool,
    size_of_mempool,
};

/// A mempool monitoring client for observing pending transactions.
///
/// This client provides methods for acquiring a snapshot of the mempool
/// and iterating through pending transactions.
///
/// # Example
///
/// ```rust,no_run
/// use ogmios_client::mempool_monitoring::MempoolMonitoringClient;
/// use ogmios_client::connection::ConnectionConfig;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = MempoolMonitoringClient::connect(ConnectionConfig::default()).await?;
///
/// // Acquire mempool snapshot
/// let slot = client.acquire_mempool().await?;
/// println!("Acquired mempool at slot {}", slot);
///
/// // Get mempool size
/// let size = client.size_of_mempool().await?;
/// println!("Mempool has {} transactions ({} bytes)",
///     size.transactions, size.bytes);
///
/// // Iterate through transactions
/// while let Some(tx) = client.next_transaction().await? {
///     println!("Transaction: {}", tx.id);
/// }
///
/// // Release when done
/// client.release_mempool().await?;
/// client.shutdown().await?;
/// # Ok(())
/// # }
/// ```
pub struct MempoolMonitoringClient {
    /// The interaction context.
    context: Arc<InteractionContext>,
}

impl MempoolMonitoringClient {
    /// Create a new mempool monitoring client from an existing context.
    pub fn new(context: InteractionContext) -> Self {
        Self {
            context: Arc::new(context),
        }
    }

    /// Connect to Ogmios and create a new mempool monitoring client.
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection configuration.
    pub async fn connect(connection: ConnectionConfig) -> Result<Self> {
        let context = create_interaction_context(InteractionContextOptions {
            connection,
            interaction_type: InteractionType::LongRunning,
            ..Default::default()
        })
        .await?;

        Ok(Self::new(context))
    }

    /// Get a reference to the interaction context.
    pub fn context(&self) -> &InteractionContext {
        &self.context
    }

    /// Acquire a snapshot of the mempool.
    ///
    /// # Returns
    ///
    /// The slot number at which the mempool was acquired.
    pub async fn acquire_mempool(&self) -> Result<Slot> {
        acquire_mempool(&self.context).await
    }

    /// Check if a transaction is in the mempool.
    ///
    /// # Arguments
    ///
    /// * `id` - The transaction ID to check.
    pub async fn has_transaction(&self, id: &str) -> Result<bool> {
        has_transaction(&self.context, id).await
    }

    /// Get the next transaction ID from the mempool.
    ///
    /// # Returns
    ///
    /// The next transaction ID, or `None` if the mempool has been exhausted.
    pub async fn next_transaction_id(&self) -> Result<Option<TransactionId>> {
        next_transaction_id(&self.context).await
    }

    /// Get the next full transaction from the mempool.
    ///
    /// # Returns
    ///
    /// The full transaction, or `None` if the mempool has been exhausted.
    pub async fn next_transaction(&self) -> Result<Option<Transaction>> {
        next_transaction(&self.context).await
    }

    /// Get the size and capacity of the mempool.
    pub async fn size_of_mempool(&self) -> Result<MempoolSizeAndCapacity> {
        size_of_mempool(&self.context).await
    }

    /// Release the acquired mempool snapshot.
    pub async fn release_mempool(&self) -> Result<()> {
        release_mempool(&self.context).await
    }

    /// Shutdown the client.
    pub async fn shutdown(&self) -> Result<()> {
        self.context.shutdown().await
    }
}

/// Create a mempool monitoring client.
///
/// This is a convenience function that creates a connection and client in one step.
pub async fn create_mempool_monitoring_client(
    connection: ConnectionConfig,
) -> Result<MempoolMonitoringClient> {
    MempoolMonitoringClient::connect(connection).await
}

/// Iterator over mempool transactions.
///
/// This struct provides an async iterator interface for mempool transactions.
pub struct MempoolTransactionIterator<'a> {
    client: &'a MempoolMonitoringClient,
    exhausted: bool,
}

impl<'a> MempoolTransactionIterator<'a> {
    /// Create a new mempool transaction iterator.
    pub fn new(client: &'a MempoolMonitoringClient) -> Self {
        Self {
            client,
            exhausted: false,
        }
    }

    /// Get the next transaction.
    pub async fn next(&mut self) -> Result<Option<Transaction>> {
        if self.exhausted {
            return Ok(None);
        }

        match self.client.next_transaction().await? {
            Some(tx) => Ok(Some(tx)),
            None => {
                self.exhausted = true;
                Ok(None)
            }
        }
    }
}
