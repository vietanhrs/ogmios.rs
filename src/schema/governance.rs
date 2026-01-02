//! Governance types for Conway era.

use serde::{Deserialize, Serialize};
use super::primitives::*;
use super::protocol::PartialProtocolParameters;

/// Constitution for Conway governance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constitution {
    /// Metadata anchor.
    pub metadata: Anchor,
    /// Guardian script hash (optional).
    #[serde(default)]
    pub guardian_script: Option<ScriptHash>,
}

/// Governance action types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum GovernanceAction {
    /// Motion of no confidence.
    #[serde(rename = "noConfidence")]
    NoConfidence {
        #[serde(default)]
        ancestor: Option<GovernanceActionId>,
    },
    /// Update constitutional committee.
    #[serde(rename = "constitutionalCommittee")]
    ConstitutionalCommittee {
        #[serde(default)]
        ancestor: Option<GovernanceActionId>,
        members: ConstitutionalCommitteeMembers,
    },
    /// Update constitution.
    #[serde(rename = "constitution")]
    Constitution {
        #[serde(default)]
        ancestor: Option<GovernanceActionId>,
        constitution: Constitution,
    },
    /// Hard fork initiation.
    #[serde(rename = "hardForkInitiation")]
    HardForkInitiation {
        #[serde(default)]
        ancestor: Option<GovernanceActionId>,
        version: super::protocol::ProtocolVersion,
    },
    /// Protocol parameters update.
    #[serde(rename = "protocolParametersUpdate")]
    ProtocolParametersUpdate {
        #[serde(default)]
        ancestor: Option<GovernanceActionId>,
        parameters: PartialProtocolParameters,
    },
    /// Treasury withdrawal.
    #[serde(rename = "treasuryWithdrawals")]
    TreasuryWithdrawals {
        withdrawals: Vec<TreasuryWithdrawal>,
    },
    /// Information action.
    #[serde(rename = "information")]
    Information,
}

/// Governance action ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceActionId {
    /// Transaction ID.
    pub transaction: TransactionId,
    /// Action index.
    pub index: u32,
}

/// Constitutional committee members.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstitutionalCommitteeMembers {
    /// Members to add with their term limits.
    #[serde(default)]
    pub added: Vec<ConstitutionalCommitteeMember>,
    /// Members to remove.
    #[serde(default)]
    pub removed: Vec<ConstitutionalCommitteeMemberCredential>,
    /// New quorum threshold.
    #[serde(default)]
    pub quorum: Option<Ratio>,
}

/// A constitutional committee member.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstitutionalCommitteeMember {
    /// Member ID (credential).
    pub id: ConstitutionalCommitteeMemberCredential,
    /// Term limit (epoch).
    pub term: Epoch,
}

/// Constitutional committee member credential.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConstitutionalCommitteeMemberCredential {
    Key { key: DigestBlake2b224 },
    Script { script: ScriptHash },
}

/// Treasury withdrawal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreasuryWithdrawal {
    /// Destination reward account.
    pub destination: RewardAccount,
    /// Amount in lovelace.
    pub amount: AdaValue,
}

/// Governance proposal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceProposal {
    /// Proposal ID.
    pub id: GovernanceActionId,
    /// The governance action.
    pub action: GovernanceAction,
    /// Deposit amount.
    pub deposit: AdaValue,
    /// Deposit return account.
    pub return_account: RewardAccount,
    /// Metadata anchor.
    #[serde(default)]
    pub metadata: Option<Anchor>,
}

/// Governance proposal state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceProposalState {
    /// The proposal.
    pub proposal: GovernanceProposal,
    /// When the proposal was proposed (epoch).
    pub proposed_in: Epoch,
    /// When the proposal expires (epoch).
    pub expires_after: Epoch,
    /// Votes cast for this proposal.
    #[serde(default)]
    pub votes: GovernanceVotes,
}

/// Votes on a governance proposal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceVotes {
    /// Stake pool votes.
    #[serde(default)]
    pub stake_pools: Vec<GovernanceVote>,
    /// DRep votes.
    #[serde(default)]
    pub delegate_representatives: Vec<GovernanceVote>,
    /// Constitutional committee votes.
    #[serde(default)]
    pub constitutional_committee: Vec<GovernanceVote>,
}

/// A governance vote.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceVote {
    /// Voter ID.
    pub voter: GovernanceVoter,
    /// The vote.
    pub vote: Vote,
    /// Optional metadata anchor.
    #[serde(default)]
    pub metadata: Option<Anchor>,
}

/// A governance voter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "camelCase")]
pub enum GovernanceVoter {
    /// Stake pool operator.
    #[serde(rename = "stakePoolOperator")]
    StakePoolOperator { id: StakePoolId },
    /// Delegate representative.
    #[serde(rename = "delegateRepresentative")]
    DelegateRepresentative {
        #[serde(flatten)]
        credential: DelegateRepresentativeCredential,
    },
    /// Constitutional committee member.
    #[serde(rename = "constitutionalCommittee")]
    ConstitutionalCommittee {
        #[serde(flatten)]
        credential: ConstitutionalCommitteeMemberCredential,
    },
}

/// Delegate representative credential.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DelegateRepresentativeCredential {
    Key { id: DigestBlake2b224 },
    Script { id: ScriptHash },
}

/// Vote choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Vote {
    Yes,
    No,
    Abstain,
}

/// Delegate representative (DRep) information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateRepresentative {
    /// DRep ID (credential).
    pub id: DelegateRepresentativeCredential,
    /// Deposit amount.
    pub deposit: AdaValue,
    /// Status.
    pub status: DRepStatus,
    /// Metadata anchor.
    #[serde(default)]
    pub metadata: Option<Anchor>,
}

/// DRep status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DRepStatus {
    Registered,
    Unregistered,
}

/// DRep summary with voting power.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateRepresentativeSummary {
    /// DRep info.
    #[serde(flatten)]
    pub drep: DelegateRepresentative,
    /// Voting power (stake delegated).
    pub voting_power: Lovelace,
}
