//! Block types for Cardano.

use serde::{Deserialize, Serialize};
use super::primitives::*;
use super::transaction::Transaction;

/// A Cardano block - can be EBB, BFT (Byron), or Praos (Shelley+).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Block {
    /// Epoch Boundary Block (Byron era).
    EBB(BlockEBB),
    /// BFT block (Byron era).
    BFT(BlockBFT),
    /// Praos block (Shelley era and later).
    Praos(BlockPraos),
}

impl Block {
    /// Get the block's slot number.
    pub fn slot(&self) -> Slot {
        match self {
            Block::EBB(b) => b.slot,
            Block::BFT(b) => b.slot,
            Block::Praos(b) => b.slot,
        }
    }

    /// Get the block's ID (hash).
    pub fn id(&self) -> &str {
        match self {
            Block::EBB(b) => &b.id,
            Block::BFT(b) => &b.id,
            Block::Praos(b) => &b.id,
        }
    }

    /// Get the block's height.
    pub fn height(&self) -> BlockHeight {
        match self {
            Block::EBB(b) => b.height,
            Block::BFT(b) => b.height,
            Block::Praos(b) => b.height,
        }
    }

    /// Get the ancestor block ID.
    pub fn ancestor(&self) -> &str {
        match self {
            Block::EBB(b) => &b.ancestor,
            Block::BFT(b) => &b.ancestor,
            Block::Praos(b) => &b.ancestor,
        }
    }

    /// Check if this is an EBB block.
    pub fn is_ebb(&self) -> bool {
        matches!(self, Block::EBB(_))
    }

    /// Check if this is a BFT block.
    pub fn is_bft(&self) -> bool {
        matches!(self, Block::BFT(_))
    }

    /// Check if this is a Praos block.
    pub fn is_praos(&self) -> bool {
        matches!(self, Block::Praos(_))
    }
}

/// Epoch Boundary Block (EBB) - Byron era.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockEBB {
    /// Block type identifier.
    #[serde(rename = "type")]
    pub block_type: String,
    /// Era (always "byron" for EBB).
    pub era: String,
    /// Block ID (hash).
    pub id: DigestBlake2b256,
    /// Ancestor block ID.
    pub ancestor: DigestBlake2b256,
    /// Slot number.
    pub slot: Slot,
    /// Block height.
    pub height: BlockHeight,
}

/// BFT block - Byron era.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockBFT {
    /// Block type identifier.
    #[serde(rename = "type")]
    pub block_type: String,
    /// Era (always "byron" for BFT).
    pub era: String,
    /// Block ID (hash).
    pub id: DigestBlake2b256,
    /// Ancestor block ID.
    pub ancestor: DigestBlake2b256,
    /// Slot number.
    pub slot: Slot,
    /// Block height.
    pub height: BlockHeight,
    /// Block size in bytes.
    pub size: BlockSize,
    /// Protocol version.
    pub protocol: ProtocolVersionByron,
    /// Block issuer information.
    pub issuer: BlockIssuerByron,
    /// Transactions in the block.
    #[serde(default)]
    pub transactions: Vec<Transaction>,
}

/// Praos block - Shelley era and later.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPraos {
    /// Block type identifier.
    #[serde(rename = "type")]
    pub block_type: String,
    /// Era (shelley, allegra, mary, alonzo, babbage, conway).
    pub era: String,
    /// Block ID (hash).
    pub id: DigestBlake2b256,
    /// Ancestor block ID.
    pub ancestor: DigestBlake2b256,
    /// Slot number.
    pub slot: Slot,
    /// Block height.
    pub height: BlockHeight,
    /// Block size in bytes.
    pub size: BlockSize,
    /// Protocol version.
    pub protocol: ProtocolVersionPraos,
    /// Block issuer information.
    pub issuer: BlockIssuerPraos,
    /// Transactions in the block.
    #[serde(default)]
    pub transactions: Vec<Transaction>,
}

/// Block size information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockSize {
    /// Size in bytes.
    pub bytes: NumberOfBytes,
}

/// Protocol version for Byron blocks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolVersionByron {
    /// Software version.
    #[serde(default)]
    pub software: Option<SoftwareVersion>,
    /// Update proposal (if any).
    #[serde(default)]
    pub update: Option<serde_json::Value>,
}

/// Software version information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareVersion {
    /// Application name.
    pub app_name: String,
    /// Version number.
    pub number: u32,
}

/// Protocol version for Praos blocks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolVersionPraos {
    /// Major version.
    pub major: u32,
    /// Minor version.
    pub minor: u32,
    /// Patch version (optional).
    #[serde(default)]
    pub patch: Option<u32>,
}

/// Block issuer for Byron blocks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockIssuerByron {
    /// Verification key hash.
    pub verification_key: VerificationKey,
}

/// Block issuer for Praos blocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockIssuerPraos {
    /// Verification key hash (pool ID).
    pub verification_key: VerificationKeyHash,
    /// VRF verification key.
    pub vrf_verification_key: VrfVerificationKey,
    /// Leader value (VRF output).
    #[serde(default)]
    pub leader_value: Option<CertifiedVrf>,
    /// Operational certificate.
    #[serde(default)]
    pub operational_certificate: Option<OperationalCertificate>,
}

/// VRF certified output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertifiedVrf {
    /// VRF output.
    pub output: String,
    /// VRF proof.
    pub proof: String,
}

/// Operational certificate for block production.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationalCertificate {
    /// KES verification key.
    pub kes_verification_key: KesVerificationKey,
    /// Counter.
    pub count: u64,
    /// KES period.
    #[serde(default)]
    pub kes_period: Option<u64>,
    /// Issue number.
    #[serde(default)]
    pub issue_number: Option<u64>,
}
