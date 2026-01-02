//! Mempool Monitoring client for Ogmios.
//!
//! This module provides functionality for monitoring the Cardano node's mempool
//! via Ogmios.

mod client;

pub use client::*;

use crate::connection::InteractionContext;
use crate::error::Result;
use crate::schema::{MempoolSizeAndCapacity, Slot, Transaction, TransactionId};
use serde::{Deserialize, Serialize};

/// Acquire a snapshot of the mempool.
///
/// This function acquires exclusive access to a snapshot of the current mempool
/// state. The snapshot remains consistent until released.
///
/// # Arguments
///
/// * `context` - The interaction context.
///
/// # Returns
///
/// The slot number at which the mempool was acquired.
pub async fn acquire_mempool(context: &InteractionContext) -> Result<Slot> {
    #[derive(Deserialize)]
    struct Response {
        slot: Slot,
    }

    let response: Response = context.request("acquireMempool", None::<()>).await?;
    Ok(response.slot)
}

/// Check if a transaction is in the mempool.
///
/// # Arguments
///
/// * `context` - The interaction context.
/// * `id` - The transaction ID to check.
///
/// # Returns
///
/// `true` if the transaction is in the mempool.
pub async fn has_transaction(context: &InteractionContext, id: &str) -> Result<bool> {
    #[derive(Serialize)]
    struct Params<'a> {
        id: &'a str,
    }

    #[derive(Deserialize)]
    struct Response {
        #[serde(rename = "hasTransaction")]
        has_transaction: bool,
    }

    let response: Response = context
        .request("hasTransaction", Some(Params { id }))
        .await?;
    Ok(response.has_transaction)
}

/// Get the next transaction from the mempool.
///
/// # Arguments
///
/// * `context` - The interaction context.
///
/// # Returns
///
/// The next transaction ID, or `None` if the mempool has been exhausted.
pub async fn next_transaction_id(context: &InteractionContext) -> Result<Option<TransactionId>> {
    #[derive(Deserialize)]
    struct Response {
        transaction: Option<TransactionWrapper>,
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TransactionWrapper {
        Id { id: TransactionId },
        Full(Transaction),
    }

    let response: Response = context.request("nextTransaction", None::<()>).await?;

    Ok(response.transaction.map(|t| match t {
        TransactionWrapper::Id { id } => id,
        TransactionWrapper::Full(tx) => tx.id,
    }))
}

/// Get the next transaction from the mempool with full details.
///
/// # Arguments
///
/// * `context` - The interaction context.
///
/// # Returns
///
/// The full transaction, or `None` if the mempool has been exhausted.
pub async fn next_transaction(context: &InteractionContext) -> Result<Option<Transaction>> {
    #[derive(Serialize)]
    struct Params {
        fields: &'static str,
    }

    #[derive(Deserialize)]
    struct Response {
        transaction: Option<Transaction>,
    }

    let response: Response = context
        .request("nextTransaction", Some(Params { fields: "all" }))
        .await?;

    Ok(response.transaction)
}

/// Get the size and capacity of the mempool.
///
/// # Arguments
///
/// * `context` - The interaction context.
///
/// # Returns
///
/// The mempool size and capacity information.
pub async fn size_of_mempool(context: &InteractionContext) -> Result<MempoolSizeAndCapacity> {
    context.request("sizeOfMempool", None::<()>).await
}

/// Release the acquired mempool snapshot.
///
/// # Arguments
///
/// * `context` - The interaction context.
pub async fn release_mempool(context: &InteractionContext) -> Result<()> {
    let _: serde_json::Value = context.request("releaseMempool", None::<()>).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_compiles() {
        // Basic compilation test
    }
}
