//! Chain Synchronization client for Ogmios.
//!
//! This module provides functionality for synchronizing with the Cardano blockchain
//! using the Ouroboros mini-protocols via Ogmios.

mod client;

pub use client::*;

use crate::connection::InteractionContext;
use crate::error::Result;
use crate::schema::{Block, Point, Tip, responses::{FindIntersectionResponse, NextBlockResponse}};
use serde::{Deserialize, Serialize};

/// Intersection result from findIntersection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Intersection {
    /// The intersection point found.
    pub point: Point,
    /// Current tip.
    pub tip: Tip,
}

/// Message handlers for chain synchronization events.
///
/// These callbacks are invoked when blocks are received or rolled back.
pub trait ChainSynchronizationMessageHandlers: Send + Sync {
    /// Called when a new block is received (roll forward).
    fn on_roll_forward(&mut self, block: Block, tip: Tip) -> Result<()>;

    /// Called when a rollback occurs (roll backward).
    fn on_roll_backward(&mut self, point: Point, tip: Tip) -> Result<()>;
}

/// Find an intersection point between the client and the node.
///
/// This function attempts to find a common point in the blockchain history
/// between the provided points and the node's chain.
///
/// # Arguments
///
/// * `context` - The interaction context.
/// * `points` - A list of points to try to intersect with.
///
/// # Returns
///
/// The intersection point and current tip if found.
pub async fn find_intersection(
    context: &InteractionContext,
    points: Vec<Point>,
) -> Result<Intersection> {
    #[derive(Serialize)]
    struct Params {
        points: Vec<Point>,
    }

    let response: FindIntersectionResponse = context
        .request("findIntersection", Some(Params { points }))
        .await?;

    if let Some(point) = response.intersection {
        Ok(Intersection {
            point,
            tip: response.tip,
        })
    } else {
        Err(crate::error::OgmiosError::IntersectionNotFound {
            tip: Some(format!("{:?}", response.tip)),
        })
    }
}

/// Request the next block from the chain.
///
/// This function requests the next block in the chain synchronization sequence.
///
/// # Arguments
///
/// * `context` - The interaction context.
///
/// # Returns
///
/// The next block response, which can be either a roll forward with a new block
/// or a roll backward indicating a rollback.
pub async fn next_block(context: &InteractionContext) -> Result<NextBlockResponse> {
    context.request("nextBlock", None::<()>).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_creation() {
        let intersection = Intersection {
            point: Point::origin(),
            tip: Tip::Origin("origin".to_string()),
        };
        assert!(matches!(intersection.point, Point::Origin(_)));
    }
}
