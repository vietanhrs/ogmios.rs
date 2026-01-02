//! Certificate types for Cardano.

use serde::{Deserialize, Serialize};
use super::primitives::*;
use super::governance::{DelegateRepresentativeCredential, ConstitutionalCommitteeMemberCredential};

/// A Cardano certificate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Certificate {
    /// Stake credential registration.
    #[serde(rename = "stakeCredentialRegistration")]
    StakeCredentialRegistration {
        credential: StakeCredential,
        #[serde(default)]
        deposit: Option<AdaValue>,
    },
    /// Stake credential deregistration.
    #[serde(rename = "stakeCredentialDeregistration")]
    StakeCredentialDeregistration {
        credential: StakeCredential,
        #[serde(default)]
        deposit: Option<AdaValue>,
    },
    /// Stake delegation.
    #[serde(rename = "stakeDelegation")]
    StakeDelegation {
        credential: StakeCredential,
        #[serde(rename = "stakePool")]
        stake_pool: StakePoolId,
    },
    /// Stake pool registration.
    #[serde(rename = "stakePoolRegistration")]
    StakePoolRegistration {
        #[serde(rename = "stakePool")]
        stake_pool: StakePool,
    },
    /// Stake pool retirement.
    #[serde(rename = "stakePoolRetirement")]
    StakePoolRetirement {
        #[serde(rename = "stakePool")]
        stake_pool: StakePoolId,
        #[serde(rename = "retirementEpoch")]
        retirement_epoch: Epoch,
    },
    /// Genesis key delegation (deprecated).
    #[serde(rename = "genesisDelegation")]
    GenesisDelegation {
        issuer: DigestBlake2b224,
        delegate: DigestBlake2b224,
        vrf: VrfVerificationKey,
    },
    /// DRep registration (Conway).
    #[serde(rename = "delegateRepresentativeRegistration")]
    DelegateRepresentativeRegistration {
        #[serde(rename = "delegateRepresentative")]
        delegate_representative: DelegateRepresentativeCredential,
        deposit: AdaValue,
        #[serde(default)]
        metadata: Option<Anchor>,
    },
    /// DRep update (Conway).
    #[serde(rename = "delegateRepresentativeUpdate")]
    DelegateRepresentativeUpdate {
        #[serde(rename = "delegateRepresentative")]
        delegate_representative: DelegateRepresentativeCredential,
        #[serde(default)]
        metadata: Option<Anchor>,
    },
    /// DRep retirement (Conway).
    #[serde(rename = "delegateRepresentativeRetirement")]
    DelegateRepresentativeRetirement {
        #[serde(rename = "delegateRepresentative")]
        delegate_representative: DelegateRepresentativeCredential,
        deposit: AdaValue,
    },
    /// Vote delegation (Conway).
    #[serde(rename = "voteDelegation")]
    VoteDelegation {
        credential: StakeCredential,
        #[serde(rename = "delegateRepresentative")]
        delegate_representative: Delegatee,
    },
    /// Stake and vote delegation (Conway).
    #[serde(rename = "stakeAndVoteDelegation")]
    StakeAndVoteDelegation {
        credential: StakeCredential,
        #[serde(rename = "stakePool")]
        stake_pool: StakePoolId,
        #[serde(rename = "delegateRepresentative")]
        delegate_representative: Delegatee,
    },
    /// Stake registration and delegation (Conway).
    #[serde(rename = "stakeCredentialRegistrationAndDelegation")]
    StakeCredentialRegistrationAndDelegation {
        credential: StakeCredential,
        #[serde(rename = "stakePool")]
        stake_pool: StakePoolId,
        deposit: AdaValue,
    },
    /// Vote registration and delegation (Conway).
    #[serde(rename = "stakeCredentialRegistrationAndVoteDelegation")]
    StakeCredentialRegistrationAndVoteDelegation {
        credential: StakeCredential,
        #[serde(rename = "delegateRepresentative")]
        delegate_representative: Delegatee,
        deposit: AdaValue,
    },
    /// Stake registration and both delegations (Conway).
    #[serde(rename = "stakeCredentialRegistrationAndBothDelegations")]
    StakeCredentialRegistrationAndBothDelegations {
        credential: StakeCredential,
        #[serde(rename = "stakePool")]
        stake_pool: StakePoolId,
        #[serde(rename = "delegateRepresentative")]
        delegate_representative: Delegatee,
        deposit: AdaValue,
    },
    /// Constitutional committee member registration (Conway).
    #[serde(rename = "constitutionalCommitteeHotKeyRegistration")]
    ConstitutionalCommitteeHotKeyRegistration {
        member: ConstitutionalCommitteeMemberCredential,
        #[serde(rename = "hotKey")]
        hot_key: ConstitutionalCommitteeMemberCredential,
    },
    /// Constitutional committee member resignation (Conway).
    #[serde(rename = "constitutionalCommitteeMemberResignation")]
    ConstitutionalCommitteeMemberResignation {
        member: ConstitutionalCommitteeMemberCredential,
        #[serde(default)]
        metadata: Option<Anchor>,
    },
}

/// Delegatee for vote delegation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Delegatee {
    /// Delegate to a specific DRep.
    DRep(DelegateRepresentativeCredential),
    /// Delegate to always abstain.
    #[serde(rename = "abstain")]
    Abstain(String),
    /// Delegate to always vote no confidence.
    #[serde(rename = "noConfidence")]
    NoConfidence(String),
}

/// Stake pool registration parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakePool {
    /// Pool ID.
    pub id: StakePoolId,
    /// VRF key hash.
    pub vrf: VrfVerificationKey,
    /// Pledge amount.
    pub pledge: AdaValue,
    /// Pool cost.
    pub cost: AdaValue,
    /// Pool margin (fee ratio).
    pub margin: Ratio,
    /// Reward account.
    pub reward_account: RewardAccount,
    /// Pool owners.
    pub owners: Vec<DigestBlake2b224>,
    /// Pool relays.
    #[serde(default)]
    pub relays: Vec<Relay>,
    /// Pool metadata.
    #[serde(default)]
    pub metadata: Option<PoolMetadata>,
}

/// Pool relay configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Relay {
    /// IP address relay.
    IpAddress {
        #[serde(rename = "ipv4")]
        ipv4: Option<String>,
        #[serde(rename = "ipv6")]
        ipv6: Option<String>,
        port: Option<u16>,
    },
    /// DNS hostname relay.
    Hostname {
        hostname: String,
        port: Option<u16>,
    },
    /// DNS SRV record relay.
    #[serde(rename = "dnsA")]
    DnsA {
        hostname: String,
    },
}

/// Pool metadata reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolMetadata {
    /// Metadata URL.
    pub url: String,
    /// Metadata hash.
    pub hash: DigestBlake2b256,
}

/// Stake pool view (for queries).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakePoolView {
    /// Pool ID.
    pub id: StakePoolId,
    /// Pool parameters.
    #[serde(flatten)]
    pub parameters: StakePool,
    /// Pool status.
    #[serde(default)]
    pub status: Option<StakePoolStatus>,
    /// Live stake (if requested).
    #[serde(default)]
    pub stake: Option<AdaValue>,
}

/// Stake pool status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StakePoolStatus {
    Active,
    Retiring,
    Retired,
}

/// Stake pool performance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakePoolPerformance {
    /// Pool ID.
    pub id: StakePoolId,
    /// Performance ratio.
    pub performance: f64,
}
