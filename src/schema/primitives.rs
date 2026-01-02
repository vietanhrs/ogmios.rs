//! Primitive types used throughout the Ogmios schema.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A 64-bit unsigned integer slot number.
pub type Slot = u64;

/// A 64-bit unsigned integer epoch number.
pub type Epoch = u64;

/// Block height as a 64-bit unsigned integer.
pub type BlockHeight = u64;

/// A lovelace amount (1 ADA = 1,000,000 lovelace).
pub type Lovelace = u64;

/// A signed lovelace delta for value changes.
pub type LovelaceDelta = i128;

/// Transaction ID as a hex-encoded string (64 characters).
pub type TransactionId = String;

/// Policy ID as a hex-encoded string (56 characters).
pub type PolicyId = String;

/// Asset name as a hex-encoded string.
pub type AssetName = String;

/// Asset quantity (can be negative for burning).
pub type AssetQuantity = i128;

/// Script hash as a hex-encoded string.
pub type ScriptHash = String;

/// Datum hash as a hex-encoded string (64 characters, Blake2b-256).
pub type DatumHash = String;

/// Pool ID as a Bech32-encoded string.
pub type PoolId = String;

/// Stake pool ID (hex-encoded hash).
pub type StakePoolId = String;

/// Verification key hash.
pub type VerificationKeyHash = String;

/// A generic digest type (hex-encoded).
pub type Digest = String;

/// Blake2b-224 digest (56 hex characters).
pub type DigestBlake2b224 = String;

/// Blake2b-256 digest (64 hex characters).
pub type DigestBlake2b256 = String;

/// A ratio represented as a string "numerator/denominator".
pub type RatioString = String;

/// Network magic number.
pub type NetworkMagic = u32;

/// Relative time in seconds.
pub type RelativeTime = f64;

/// UTC time as ISO 8601 string.
pub type UtcTime = String;

/// Number of bytes.
pub type NumberOfBytes = u64;

/// The origin point of the blockchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Origin;

impl Origin {
    pub const ORIGIN: &'static str = "origin";
}

/// A point on the blockchain, either origin or a specific slot/hash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Point {
    /// The origin point.
    Origin(String),
    /// A specific point with slot and block ID.
    Point {
        slot: Slot,
        id: DigestBlake2b256,
    },
}

impl Point {
    /// Create an origin point.
    pub fn origin() -> Self {
        Point::Origin("origin".to_string())
    }

    /// Create a point at a specific slot and block ID.
    pub fn at(slot: Slot, id: impl Into<String>) -> Self {
        Point::Point { slot, id: id.into() }
    }
}

/// The tip of the blockchain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tip {
    /// The origin (empty chain).
    Origin(String),
    /// A specific tip with slot, ID, and height.
    Tip {
        slot: Slot,
        id: DigestBlake2b256,
        height: BlockHeight,
    },
}

/// A rational number represented as numerator and denominator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ratio {
    pub numerator: u64,
    pub denominator: u64,
}

impl Ratio {
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self { numerator, denominator }
    }

    pub fn to_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

/// Assets as a map of policy ID to a map of asset name to quantity.
pub type Assets = HashMap<PolicyId, HashMap<AssetName, AssetQuantity>>;

/// Value containing ADA and optional multi-assets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// ADA only value.
    AdaOnly {
        ada: AdaValue,
    },
    /// Value with ADA and other assets.
    WithAssets {
        ada: AdaValue,
        #[serde(flatten)]
        assets: Assets,
    },
}

/// ADA value container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaValue {
    pub lovelace: Lovelace,
}

impl Value {
    /// Create an ADA-only value.
    pub fn ada_only(lovelace: Lovelace) -> Self {
        Value::AdaOnly {
            ada: AdaValue { lovelace },
        }
    }

    /// Get the lovelace amount.
    pub fn lovelace(&self) -> Lovelace {
        match self {
            Value::AdaOnly { ada } => ada.lovelace,
            Value::WithAssets { ada, .. } => ada.lovelace,
        }
    }
}

/// A Cardano address (Bech32 or Base58 encoded).
pub type Address = String;

/// A stake address (Bech32 encoded, starts with "stake").
pub type StakeAddress = String;

/// A reward account (same as stake address).
pub type RewardAccount = String;

/// Nonce for protocol parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Nonce {
    /// Neutral nonce.
    Neutral,
    /// Specific nonce value.
    #[serde(untagged)]
    Value(String),
}

/// A verification key (hex-encoded).
pub type VerificationKey = String;

/// An extended verification key.
pub type ExtendedVerificationKey = String;

/// A KES verification key.
pub type KesVerificationKey = String;

/// A VRF verification key.
pub type VrfVerificationKey = String;

/// A signature (hex-encoded).
pub type Signature = String;

/// Metadata labels type.
pub type MetadataLabels = HashMap<String, Metadatum>;

/// Metadata value that can be various types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Metadatum {
    /// Integer value.
    Int(i128),
    /// Byte string (hex-encoded).
    Bytes(String),
    /// Text string.
    String(String),
    /// List of metadatums.
    List(Vec<Metadatum>),
    /// Map of metadatums.
    Map(Vec<MetadatumMapEntry>),
}

/// A key-value entry in a metadatum map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadatumMapEntry {
    pub k: Metadatum,
    pub v: Metadatum,
}

/// Anchor for governance actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Anchor {
    pub url: String,
    #[serde(rename = "hash")]
    pub content_hash: DigestBlake2b256,
}

/// Credential origin - either from a key or a script.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CredentialOrigin {
    Key,
    Script,
}

/// A stake credential.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StakeCredential {
    /// Key-based credential.
    Key { key: DigestBlake2b224 },
    /// Script-based credential.
    Script { script: ScriptHash },
}

/// A payment credential.
pub type PaymentCredential = StakeCredential;

/// Nullable type wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Nullable<T> {
    Null,
    Value(T),
}

impl<T> Nullable<T> {
    pub fn is_null(&self) -> bool {
        matches!(self, Nullable::Null)
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            Nullable::Null => None,
            Nullable::Value(v) => Some(v),
        }
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => Nullable::Value(v),
            None => Nullable::Null,
        }
    }
}
