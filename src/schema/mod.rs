//! Cardano schema types for Ogmios.
//!
//! This module contains all the type definitions that correspond to the
//! Ogmios JSON schema, mirroring the `@cardano-ogmios/schema` TypeScript package.

mod primitives;
mod block;
mod transaction;
mod protocol;
mod governance;
mod certificates;
mod scripts;
mod genesis;
mod era;
mod network;
mod jsonrpc;

// Primitives - export all (including Value, Address, etc.)
pub use primitives::*;

// Block types
pub use block::{
    Block, BlockBFT, BlockEBB, BlockPraos, BlockIssuerByron, BlockIssuerPraos,
    BlockSize, CertifiedVrf, OperationalCertificate, ProtocolVersionByron,
    ProtocolVersionPraos, SoftwareVersion,
};

// Transaction types
pub use transaction::{
    BootstrapWitness, EvaluationResult, ExUnits, InputSource, KeyWitness, Metadata,
    ScriptPurpose, Transaction, TransactionInput, TransactionOutput,
    TransactionOutputReference, Utxo, ValidatorIndex, Witnesses,
};

// Protocol types (excluding BlockSize which is already exported from block)
pub use protocol::{
    ConstitutionalCommitteeThresholds, CostModels, DelegateRepresentativeVotingThresholds,
    DRepProtocolParametersUpdateThresholds, MinFeeReferenceScripts, PartialProtocolParameters,
    ProposedProtocolParameters, ProtocolParameters, ProtocolParametersUpdateThresholds,
    ProtocolVersion, ScriptExecutionPrices, StakePoolVotingThresholds,
};

// Governance types
pub use governance::{
    ConstitutionalCommitteeMembers, ConstitutionalCommitteeMember,
    ConstitutionalCommitteeMemberCredential, Constitution, DelegateRepresentative,
    DelegateRepresentativeCredential, DelegateRepresentativeSummary, DRepStatus,
    GovernanceAction, GovernanceActionId, GovernanceProposal, GovernanceProposalState,
    GovernanceVote, GovernanceVoter, GovernanceVotes, TreasuryWithdrawal, Vote,
};

// Certificate types (excluding ConstitutionalCommitteeMemberCredential which is from governance)
pub use certificates::{
    Certificate, Delegatee, PoolMetadata, Relay, StakePool, StakePoolPerformance,
    StakePoolStatus, StakePoolView,
};

// Script types
pub use scripts::{Datum, Language, NativeScript, OutputReference, Redeemer, RedeemerPurpose, Script, ScriptReference};

// Genesis types
pub use genesis::{
    BootstrapProtocolParameters, ConstitutionalCommitteeConfig, GenesisAlonzo,
    GenesisByron, GenesisConfiguration, GenesisConway, GenesisDelegate,
    GenesisShelley, GenesisStakePools, InitialDelegate,
};

// Era types
pub use era::{Era, EraBound, EraParameters, EraStart, EraSummary, EraWithGenesis};

// Network types
pub use network::{
    LiveStakeDistributionEntry, MempoolSizeAndCapacity, Network, ProjectedRewards,
    RewardAccountSummary, RuntimeStats, ServerHealth, ServerMetrics, SessionDurations,
};

// JSON-RPC types
pub use jsonrpc::{
    error_codes, responses, JsonRpcError, JsonRpcRequest, JsonRpcResponse, JSONRPC_VERSION,
};
