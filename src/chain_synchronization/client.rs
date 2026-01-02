//! Chain Synchronization client implementation.

use crate::connection::{
    create_interaction_context, ConnectionConfig, InteractionContext, InteractionContextOptions,
    InteractionType,
};
use crate::error::Result;
use crate::schema::{Block, Point, Tip, responses::NextBlockResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info, trace};

use super::{find_intersection, next_block, ChainSynchronizationMessageHandlers, Intersection};

/// Options for creating a chain synchronization client.
#[derive(Default)]
pub struct ChainSynchronizationClientOptions {
    /// Process blocks sequentially (one at a time).
    pub sequential: bool,
}

/// A chain synchronization client for following the Cardano blockchain.
///
/// This client implements the Ouroboros chain-sync mini-protocol, allowing
/// you to follow the blockchain from a specific point and receive notifications
/// about new blocks and rollbacks.
///
/// # Example
///
/// ```rust,no_run
/// use ogmios_client::chain_synchronization::{
///     ChainSynchronizationClient,
///     ChainSynchronizationMessageHandlers,
/// };
/// use ogmios_client::connection::{ConnectionConfig, create_interaction_context, InteractionContextOptions, InteractionType};
/// use ogmios_client::schema::{Block, Point, Tip};
/// use ogmios_client::error::Result;
///
/// struct MyHandler;
///
/// impl ChainSynchronizationMessageHandlers for MyHandler {
///     fn on_roll_forward(&mut self, block: Block, tip: Tip) -> Result<()> {
///         println!("New block at slot {}", block.slot());
///         Ok(())
///     }
///
///     fn on_roll_backward(&mut self, point: Point, tip: Tip) -> Result<()> {
///         println!("Rollback to {:?}", point);
///         Ok(())
///     }
/// }
///
/// # async fn example() -> Result<()> {
/// let context = create_interaction_context(InteractionContextOptions {
///     connection: ConnectionConfig::default(),
///     interaction_type: InteractionType::LongRunning,
///     ..Default::default()
/// }).await?;
///
/// let client = ChainSynchronizationClient::new(context, MyHandler, Default::default()).await?;
///
/// // Start syncing from origin
/// let intersection = client.resume(Some(vec![Point::origin()]), None).await?;
/// println!("Started at {:?}", intersection.point);
/// # Ok(())
/// # }
/// ```
pub struct ChainSynchronizationClient<H: ChainSynchronizationMessageHandlers> {
    /// The interaction context.
    context: Arc<InteractionContext>,
    /// Message handlers.
    handlers: Arc<Mutex<H>>,
    /// Client options.
    options: ChainSynchronizationClientOptions,
    /// Whether the client is currently running.
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl<H: ChainSynchronizationMessageHandlers + 'static> ChainSynchronizationClient<H> {
    /// Create a new chain synchronization client.
    ///
    /// # Arguments
    ///
    /// * `context` - The interaction context.
    /// * `handlers` - Message handlers for block events.
    /// * `options` - Client options.
    pub async fn new(
        context: InteractionContext,
        handlers: H,
        options: ChainSynchronizationClientOptions,
    ) -> Result<Self> {
        Ok(Self {
            context: Arc::new(context),
            handlers: Arc::new(Mutex::new(handlers)),
            options,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

    /// Get a reference to the interaction context.
    pub fn context(&self) -> &InteractionContext {
        &self.context
    }

    /// Check if the client is currently running.
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Resume chain synchronization from given points.
    ///
    /// This function finds an intersection with the provided points and starts
    /// synchronizing from that point.
    ///
    /// # Arguments
    ///
    /// * `points` - Optional list of points to try to intersect with. If not provided,
    ///              starts from the origin.
    /// * `in_flight` - Optional number of blocks to request in parallel.
    ///
    /// # Returns
    ///
    /// The intersection point that was found.
    pub async fn resume(
        &self,
        points: Option<Vec<Point>>,
        in_flight: Option<u32>,
    ) -> Result<Intersection> {
        let points = points.unwrap_or_else(|| vec![Point::origin()]);
        let intersection = find_intersection(&self.context, points).await?;

        info!(
            "Chain sync resumed from {:?}, tip at {:?}",
            intersection.point, intersection.tip
        );

        // Start the sync loop
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);

        let context = self.context.clone();
        let handlers = self.handlers.clone();
        let running = self.running.clone();
        let sequential = self.options.sequential;

        tokio::spawn(async move {
            if let Err(e) = run_sync_loop(context, handlers, running.clone(), sequential).await {
                error!("Chain sync error: {}", e);
            }
            running.store(false, std::sync::atomic::Ordering::SeqCst);
        });

        Ok(intersection)
    }

    /// Shutdown the chain synchronization client.
    pub async fn shutdown(&self) -> Result<()> {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        self.context.shutdown().await
    }
}

/// Run the synchronization loop.
async fn run_sync_loop<H: ChainSynchronizationMessageHandlers>(
    context: Arc<InteractionContext>,
    handlers: Arc<Mutex<H>>,
    running: Arc<std::sync::atomic::AtomicBool>,
    _sequential: bool,
) -> Result<()> {
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        if !context.is_socket_open() {
            debug!("Socket closed, stopping sync loop");
            break;
        }

        match next_block(&context).await {
            Ok(response) => {
                let mut handlers = handlers.lock().await;
                match response {
                    NextBlockResponse::Forward { block, tip } => {
                        trace!("Received block at slot {}", block.slot());
                        if let Err(e) = handlers.on_roll_forward(block, tip) {
                            error!("Error in roll forward handler: {}", e);
                            return Err(e);
                        }
                    }
                    NextBlockResponse::Backward { point, tip } => {
                        debug!("Rollback to {:?}", point);
                        if let Err(e) = handlers.on_roll_backward(point, tip) {
                            error!("Error in roll backward handler: {}", e);
                            return Err(e);
                        }
                    }
                }
            }
            Err(e) => {
                if running.load(std::sync::atomic::Ordering::SeqCst) {
                    error!("Error getting next block: {}", e);
                    return Err(e);
                }
                break;
            }
        }
    }

    Ok(())
}

/// Create a chain synchronization client.
///
/// This is a convenience function that creates an interaction context and
/// a chain synchronization client in one step.
///
/// # Arguments
///
/// * `connection` - Connection configuration.
/// * `handlers` - Message handlers for block events.
/// * `options` - Optional client options.
///
/// # Returns
///
/// A new chain synchronization client.
pub async fn create_chain_synchronization_client<H: ChainSynchronizationMessageHandlers + 'static>(
    connection: ConnectionConfig,
    handlers: H,
    options: Option<ChainSynchronizationClientOptions>,
) -> Result<ChainSynchronizationClient<H>> {
    let context = create_interaction_context(InteractionContextOptions {
        connection,
        interaction_type: InteractionType::LongRunning,
        ..Default::default()
    })
    .await?;

    ChainSynchronizationClient::new(context, handlers, options.unwrap_or_default()).await
}

/// A simple handler that collects blocks into a vector.
///
/// Useful for testing or batch processing.
pub struct CollectingHandler {
    /// Collected blocks.
    pub blocks: Vec<Block>,
    /// Points where rollbacks occurred.
    pub rollbacks: Vec<Point>,
    /// Maximum blocks to collect (None for unlimited).
    pub max_blocks: Option<usize>,
}

impl CollectingHandler {
    /// Create a new collecting handler.
    pub fn new(max_blocks: Option<usize>) -> Self {
        Self {
            blocks: Vec::new(),
            rollbacks: Vec::new(),
            max_blocks,
        }
    }

    /// Check if the handler has reached the maximum block count.
    pub fn is_complete(&self) -> bool {
        self.max_blocks.map_or(false, |max| self.blocks.len() >= max)
    }
}

impl ChainSynchronizationMessageHandlers for CollectingHandler {
    fn on_roll_forward(&mut self, block: Block, _tip: Tip) -> Result<()> {
        self.blocks.push(block);
        Ok(())
    }

    fn on_roll_backward(&mut self, point: Point, _tip: Tip) -> Result<()> {
        self.rollbacks.push(point);
        Ok(())
    }
}

/// A handler that calls closures for each event.
///
/// This is useful for simple use cases where you don't need a full struct.
pub struct FnHandler<F, B>
where
    F: FnMut(Block, Tip) -> Result<()> + Send + Sync,
    B: FnMut(Point, Tip) -> Result<()> + Send + Sync,
{
    on_forward: F,
    on_backward: B,
}

impl<F, B> FnHandler<F, B>
where
    F: FnMut(Block, Tip) -> Result<()> + Send + Sync,
    B: FnMut(Point, Tip) -> Result<()> + Send + Sync,
{
    /// Create a new function-based handler.
    pub fn new(on_forward: F, on_backward: B) -> Self {
        Self {
            on_forward,
            on_backward,
        }
    }
}

impl<F, B> ChainSynchronizationMessageHandlers for FnHandler<F, B>
where
    F: FnMut(Block, Tip) -> Result<()> + Send + Sync,
    B: FnMut(Point, Tip) -> Result<()> + Send + Sync,
{
    fn on_roll_forward(&mut self, block: Block, tip: Tip) -> Result<()> {
        (self.on_forward)(block, tip)
    }

    fn on_roll_backward(&mut self, point: Point, tip: Tip) -> Result<()> {
        (self.on_backward)(point, tip)
    }
}
