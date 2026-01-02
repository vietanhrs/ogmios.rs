//! Transaction Submission client for Ogmios.
//!
//! This module provides functionality for submitting and evaluating transactions
//! on the Cardano blockchain via Ogmios.

mod client;

pub use client::*;

use crate::connection::InteractionContext;
use crate::error::{OgmiosError, Result};
use crate::schema::{EvaluationResult, TransactionId, Utxo};
use serde::{Deserialize, Serialize};

/// Submit a transaction to the network.
///
/// # Arguments
///
/// * `context` - The interaction context.
/// * `cbor` - The CBOR-encoded transaction (hex string).
///
/// # Returns
///
/// The transaction ID if successful.
///
/// # Example
///
/// ```rust,no_run
/// use ogmios_client::transaction_submission::submit_transaction;
/// use ogmios_client::connection::{create_interaction_context, InteractionContextOptions};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = create_interaction_context(InteractionContextOptions::default()).await?;
/// let tx_cbor = "84a400..."; // Your signed transaction CBOR
/// let tx_id = submit_transaction(&context, tx_cbor).await?;
/// println!("Transaction submitted: {}", tx_id);
/// # Ok(())
/// # }
/// ```
pub async fn submit_transaction(
    context: &InteractionContext,
    cbor: &str,
) -> Result<TransactionId> {
    #[derive(Serialize)]
    struct Params<'a> {
        transaction: Transaction<'a>,
    }

    #[derive(Serialize)]
    struct Transaction<'a> {
        cbor: &'a str,
    }

    #[derive(Deserialize)]
    struct Response {
        transaction: TransactionWrapper,
    }

    #[derive(Deserialize)]
    struct TransactionWrapper {
        id: TransactionId,
    }

    let response: Response = context
        .request(
            "submitTransaction",
            Some(Params {
                transaction: Transaction { cbor },
            }),
        )
        .await?;

    Ok(response.transaction.id)
}

/// Evaluate a transaction to get execution costs.
///
/// This function evaluates a transaction without submitting it, returning
/// the execution costs for each Plutus script in the transaction.
///
/// # Arguments
///
/// * `context` - The interaction context.
/// * `cbor` - The CBOR-encoded transaction (hex string).
/// * `additional_utxo` - Optional additional UTXOs to use for evaluation.
///
/// # Returns
///
/// A list of evaluation results for each script in the transaction.
///
/// # Example
///
/// ```rust,no_run
/// use ogmios_client::transaction_submission::evaluate_transaction;
/// use ogmios_client::connection::{create_interaction_context, InteractionContextOptions};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let context = create_interaction_context(InteractionContextOptions::default()).await?;
/// let tx_cbor = "84a400..."; // Your transaction CBOR
/// let results = evaluate_transaction(&context, tx_cbor, None).await?;
/// for result in results {
///     println!("Validator {:?}: {} mem, {} cpu",
///         result.validator,
///         result.budget.memory,
///         result.budget.cpu
///     );
/// }
/// # Ok(())
/// # }
/// ```
pub async fn evaluate_transaction(
    context: &InteractionContext,
    cbor: &str,
    additional_utxo: Option<Vec<Utxo>>,
) -> Result<Vec<EvaluationResult>> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Params<'a> {
        transaction: Transaction<'a>,
        #[serde(skip_serializing_if = "Option::is_none")]
        additional_utxo: Option<Vec<Utxo>>,
    }

    #[derive(Serialize)]
    struct Transaction<'a> {
        cbor: &'a str,
    }

    let response: serde_json::Value = context
        .request(
            "evaluateTransaction",
            Some(Params {
                transaction: Transaction { cbor },
                additional_utxo,
            }),
        )
        .await?;

    // The response can be either a list of results or an error
    if let Some(arr) = response.as_array() {
        let results: Vec<EvaluationResult> = arr
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        Ok(results)
    } else if let Some(obj) = response.as_object() {
        if obj.contains_key("error") {
            return Err(OgmiosError::EvaluationError(
                serde_json::to_string(&response).unwrap_or_default(),
            ));
        }
        // Single result
        let result: EvaluationResult = serde_json::from_value(response)?;
        Ok(vec![result])
    } else {
        Err(OgmiosError::InvalidResponse {
            message: "Unexpected evaluation response format".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_compiles() {
        // Basic compilation test
    }
}
