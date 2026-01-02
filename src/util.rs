//! Utility functions for the Ogmios client.
//!
//! This module provides various helper functions for working with Cardano data types.

use crate::schema::{Block, Datum, Lovelace, Point, Script, TransactionOutput, Value};

/// Constant output serialization overhead (160 bytes).
///
/// This is an approximation of the memory overhead for output serialization.
pub const CONSTANT_OUTPUT_SERIALIZATION_OVERHEAD: u64 = 160;

/// Check if a block is an Epoch Boundary Block (EBB).
///
/// EBB blocks are special blocks in the Byron era that mark epoch boundaries.
///
/// # Example
///
/// ```rust
/// use ogmios_client::util::is_block_ebb;
/// use ogmios_client::schema::Block;
///
/// fn check_block(block: &Block) {
///     if is_block_ebb(block) {
///         println!("This is an EBB block");
///     }
/// }
/// ```
pub fn is_block_ebb(block: &Block) -> bool {
    matches!(block, Block::EBB(_))
}

/// Check if a block is a BFT block (Byron era).
///
/// BFT blocks use the Byzantine Fault Tolerant consensus mechanism
/// from the Byron era.
pub fn is_block_bft(block: &Block) -> bool {
    matches!(block, Block::BFT(_))
}

/// Check if a block is a Praos block (Shelley+ era).
///
/// Praos blocks use the Ouroboros Praos consensus mechanism
/// from Shelley era onwards.
pub fn is_block_praos(block: &Block) -> bool {
    matches!(block, Block::Praos(_))
}

/// Check if a value is an object.
pub fn is_object<T>(_value: &T) -> bool {
    // In Rust, this is always true for struct types
    true
}

/// Calculate the size of a UTXO for minimum lovelace calculations.
///
/// This function computes the ledger-visible output size, which is used
/// to calculate the minimum amount of ADA required for a UTXO.
///
/// # Arguments
///
/// * `output` - The transaction output to calculate the size for.
///
/// # Returns
///
/// The size in bytes.
///
/// # Example
///
/// ```rust
/// use ogmios_client::util::utxo_size;
/// use ogmios_client::schema::transaction::TransactionOutput;
/// use ogmios_client::schema::primitives::Value;
///
/// fn calculate_min_ada(output: &TransactionOutput) {
///     let size = utxo_size(output);
///     println!("UTXO size: {} bytes", size);
/// }
/// ```
pub fn utxo_size(output: &TransactionOutput) -> u64 {
    let address_size = size_of_address(&output.address);
    let value_size = size_of_value(&output.value);

    let datum_size = if let Some(ref datum) = output.datum {
        size_of_inline_datum(datum)
    } else if let Some(ref _datum_hash) = output.datum_hash {
        size_of_datum_hash()
    } else {
        0
    };

    let script_size = if let Some(ref script) = output.script {
        size_of_script(script)
    } else {
        0
    };

    // Base overhead + components
    CONSTANT_OUTPUT_SERIALIZATION_OVERHEAD + address_size + value_size + datum_size + script_size
}

/// Calculate the size of a CBOR variable-length integer.
fn size_of_integer(value: u64) -> u64 {
    if value < 24 {
        1
    } else if value < 256 {
        2
    } else if value < 65536 {
        3
    } else if value < 4294967296 {
        5
    } else {
        9
    }
}

/// Calculate the size of definite-length bytes.
fn size_of_bytes_def(len: u64) -> u64 {
    size_of_integer(len) + len
}

/// Calculate the size of definite-length array/map overhead.
fn size_of_array_def(len: u64) -> u64 {
    size_of_integer(len)
}

/// Calculate the size of an address.
fn size_of_address(address: &str) -> u64 {
    // Address is typically bech32 or base58 encoded
    // The actual CBOR size depends on the decoded bytes
    // This is an approximation based on common address sizes
    let len = address.len() as u64;

    // Bech32 addresses decode to about 57-58 bytes for most addresses
    // Base58 (Byron) addresses are longer
    if address.starts_with("addr") || address.starts_with("stake") {
        // Shelley address - approximately 57 bytes when decoded
        size_of_bytes_def(57)
    } else {
        // Byron address - use length estimate
        size_of_bytes_def(len / 2)
    }
}

/// Calculate the size of a value.
fn size_of_value(value: &Value) -> u64 {
    match value {
        Value::AdaOnly { ada } => {
            // Just the lovelace amount
            size_of_integer(ada.lovelace)
        }
        Value::WithAssets { ada, assets } => {
            // Map with ADA + multi-assets
            let mut size = size_of_array_def(2); // [lovelace, multiasset_map]
            size += size_of_integer(ada.lovelace);

            // Multi-asset map
            size += size_of_array_def(assets.len() as u64);
            for (_policy_id, asset_map) in assets {
                // Policy ID is 28 bytes (224 bits)
                size += size_of_bytes_def(28);
                // Asset name -> quantity map
                size += size_of_array_def(asset_map.len() as u64);
                for (asset_name, quantity) in asset_map {
                    // Asset name (variable length, hex encoded so divide by 2)
                    size += size_of_bytes_def(asset_name.len() as u64 / 2);
                    // Quantity
                    size += size_of_integer(*quantity as u64);
                }
            }

            size
        }
    }
}

/// Calculate the size of an inline datum.
fn size_of_inline_datum(datum: &Datum) -> u64 {
    match datum {
        Datum::Cbor(cbor) => {
            // CBOR hex string - actual size is half the length
            let datum_bytes = cbor.len() as u64 / 2;
            // Tag (24) + length prefix + data
            1 + size_of_bytes_def(datum_bytes)
        }
        Datum::Value(_) => {
            // Estimate for JSON value - this is approximate
            64
        }
    }
}

/// Calculate the size of a datum hash reference.
fn size_of_datum_hash() -> u64 {
    // Datum hash is 32 bytes (256 bits)
    size_of_bytes_def(32)
}

/// Calculate the size of a script reference.
fn size_of_script(script: &Script) -> u64 {
    match script {
        Script::Native { cbor, .. } => {
            if let Some(cbor) = cbor {
                size_of_bytes_def(cbor.len() as u64 / 2)
            } else {
                // Estimate for native script without CBOR
                32
            }
        }
        Script::PlutusV1 { cbor } | Script::PlutusV2 { cbor } | Script::PlutusV3 { cbor } => {
            // CBOR hex string
            size_of_bytes_def(cbor.len() as u64 / 2)
        }
    }
}

/// Parse a point from a string or structured format.
///
/// This function handles various point representations used in the Ogmios API.
pub fn parse_point(value: &serde_json::Value) -> Option<Point> {
    if let Some(s) = value.as_str() {
        if s == "origin" {
            return Some(Point::origin());
        }
    }

    if let Some(obj) = value.as_object() {
        if let (Some(slot), Some(id)) = (obj.get("slot"), obj.get("id")) {
            if let (Some(slot), Some(id)) = (slot.as_u64(), id.as_str()) {
                return Some(Point::at(slot, id));
            }
        }
    }

    None
}

/// Format a lovelace amount as ADA with decimals.
///
/// # Example
///
/// ```rust
/// use ogmios_client::util::lovelace_to_ada;
///
/// let ada = lovelace_to_ada(1_500_000);
/// assert_eq!(ada, "1.500000");
/// ```
pub fn lovelace_to_ada(lovelace: Lovelace) -> String {
    let ada = lovelace as f64 / 1_000_000.0;
    format!("{:.6}", ada)
}

/// Convert ADA to lovelace.
///
/// # Example
///
/// ```rust
/// use ogmios_client::util::ada_to_lovelace;
///
/// let lovelace = ada_to_lovelace(1.5);
/// assert_eq!(lovelace, 1_500_000);
/// ```
pub fn ada_to_lovelace(ada: f64) -> Lovelace {
    (ada * 1_000_000.0) as Lovelace
}

/// Hex encode bytes.
pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Hex decode a string.
pub fn hex_decode(s: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lovelace_to_ada() {
        assert_eq!(lovelace_to_ada(1_000_000), "1.000000");
        assert_eq!(lovelace_to_ada(1_500_000), "1.500000");
        assert_eq!(lovelace_to_ada(500_000), "0.500000");
    }

    #[test]
    fn test_ada_to_lovelace() {
        assert_eq!(ada_to_lovelace(1.0), 1_000_000);
        assert_eq!(ada_to_lovelace(1.5), 1_500_000);
        assert_eq!(ada_to_lovelace(0.5), 500_000);
    }

    #[test]
    fn test_size_of_integer() {
        assert_eq!(size_of_integer(0), 1);
        assert_eq!(size_of_integer(23), 1);
        assert_eq!(size_of_integer(24), 2);
        assert_eq!(size_of_integer(255), 2);
        assert_eq!(size_of_integer(256), 3);
    }

    #[test]
    fn test_hex_encode_decode() {
        let bytes = vec![0xde, 0xad, 0xbe, 0xef];
        let encoded = hex_encode(&bytes);
        assert_eq!(encoded, "deadbeef");

        let decoded = hex_decode(&encoded).unwrap();
        assert_eq!(decoded, bytes);
    }

    #[test]
    fn test_parse_point() {
        let origin = serde_json::json!("origin");
        assert_eq!(parse_point(&origin), Some(Point::origin()));

        let point = serde_json::json!({
            "slot": 12345,
            "id": "abc123"
        });
        assert_eq!(parse_point(&point), Some(Point::at(12345, "abc123")));
    }
}
