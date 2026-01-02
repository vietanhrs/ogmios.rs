//! Transaction Submission client implementation.

use crate::connection::{
    create_interaction_context, ConnectionConfig, InteractionContext, InteractionContextOptions,
    InteractionType,
};
use crate::error::Result;
use crate::schema::{EvaluationResult, TransactionId, Utxo};
use std::sync::Arc;

use super::{evaluate_transaction, submit_transaction};

/// A transaction submission client for submitting and evaluating transactions.
///
/// This client provides methods for submitting signed transactions to the
/// Cardano network and evaluating transaction execution costs.
///
/// # Example
///
/// ```rust,no_run
/// use ogmios_client::transaction_submission::TransactionSubmissionClient;
/// use ogmios_client::connection::ConnectionConfig;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = TransactionSubmissionClient::connect(ConnectionConfig::default()).await?;
///
/// // Evaluate a transaction
/// let tx_cbor = "84a400...";
/// let costs = client.evaluate_transaction(tx_cbor, None).await?;
/// println!("Execution costs: {:?}", costs);
///
/// // Submit a transaction
/// let tx_id = client.submit_transaction(tx_cbor).await?;
/// println!("Submitted: {}", tx_id);
///
/// client.shutdown().await?;
/// # Ok(())
/// # }
/// ```
pub struct TransactionSubmissionClient {
    /// The interaction context.
    context: Arc<InteractionContext>,
}

impl TransactionSubmissionClient {
    /// Create a new transaction submission client from an existing context.
    pub fn new(context: InteractionContext) -> Self {
        Self {
            context: Arc::new(context),
        }
    }

    /// Connect to Ogmios and create a new transaction submission client.
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

    /// Evaluate a transaction to get execution costs.
    ///
    /// # Arguments
    ///
    /// * `cbor` - The CBOR-encoded transaction (hex string).
    /// * `additional_utxo` - Optional additional UTXOs to use for evaluation.
    ///
    /// # Returns
    ///
    /// A list of evaluation results for each script in the transaction.
    pub async fn evaluate_transaction(
        &self,
        cbor: &str,
        additional_utxo: Option<Vec<Utxo>>,
    ) -> Result<Vec<EvaluationResult>> {
        evaluate_transaction(&self.context, cbor, additional_utxo).await
    }

    /// Submit a transaction to the network.
    ///
    /// # Arguments
    ///
    /// * `cbor` - The CBOR-encoded signed transaction (hex string).
    ///
    /// # Returns
    ///
    /// The transaction ID if successful.
    pub async fn submit_transaction(&self, cbor: &str) -> Result<TransactionId> {
        submit_transaction(&self.context, cbor).await
    }

    /// Shutdown the client.
    pub async fn shutdown(&self) -> Result<()> {
        self.context.shutdown().await
    }
}

/// Create a transaction submission client.
///
/// This is a convenience function that creates a connection and client in one step.
pub async fn create_transaction_submission_client(
    connection: ConnectionConfig,
) -> Result<TransactionSubmissionClient> {
    TransactionSubmissionClient::connect(connection).await
}
