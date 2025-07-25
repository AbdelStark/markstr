//! # Utility Functions
//!
//! Common utility functions for Bitcoin operations and data conversion.

use crate::{error::Result, MarketError};
use bitcoin::{Address, Network};
use sha2::{Digest, Sha256};
use std::str::FromStr;

/// Generate a random market ID
pub fn generate_market_id() -> String {
    use bitcoin::secp256k1::rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let bytes: [u8; 4] = rng.gen();
    hex::encode(bytes).to_uppercase()
}

/// Hash a message using SHA256
pub fn sha256_hash(message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
}

/// Validate a Bitcoin address for the specified network
pub fn validate_address(address: &str, network: Network) -> bool {
    Address::from_str(address)
        .map(|addr| addr.is_valid_for_network(network))
        .unwrap_or(false)
}

/// Convert satoshis to Bitcoin
pub fn satoshi_to_btc(satoshi: u64) -> f64 {
    satoshi as f64 / 100_000_000.0
}

/// Convert Bitcoin to satoshis
pub fn btc_to_satoshi(btc: f64) -> u64 {
    (btc * 100_000_000.0) as u64
}

/// Verify a signature (placeholder implementation)
pub fn verify_signature(message: &str, signature: &str, pubkey: &str) -> Result<bool> {
    // Validate message is not empty
    if message.is_empty() {
        return Err(MarketError::InvalidSignature(
            "Message cannot be empty".to_string(),
        ));
    }

    // Validate signature is hex and 64 bytes (128 hex chars)
    if signature.len() != 128 {
        return Err(MarketError::InvalidSignature(
            "Signature must be 64 bytes (128 hex characters)".to_string(),
        ));
    }

    if hex::decode(signature).is_err() {
        return Err(MarketError::InvalidSignature(
            "Invalid signature hex encoding".to_string(),
        ));
    }

    // Validate pubkey is hex and 32 bytes (64 hex chars)
    if pubkey.len() != 64 {
        return Err(MarketError::InvalidSignature(
            "Public key must be 32 bytes (64 hex characters)".to_string(),
        ));
    }

    if hex::decode(pubkey).is_err() {
        return Err(MarketError::InvalidSignature(
            "Invalid public key hex encoding".to_string(),
        ));
    }

    // In a real implementation, this would:
    // 1. Parse the signature and pubkey
    // 2. Hash the message
    // 3. Verify the signature against the hash using secp256k1
    // 4. Return the verification result

    // For now, return true for properly formatted inputs
    Ok(true)
}

/// Network enum to u8 conversion
pub fn network_to_u8(network: Network) -> u8 {
    match network {
        Network::Bitcoin => 0,
        Network::Testnet => 1,
        Network::Signet => 2,
        Network::Regtest => 3,
    }
}

/// u8 to Network conversion
pub fn u8_to_network(network: u8) -> Result<Network> {
    match network {
        0 => Ok(Network::Bitcoin),
        1 => Ok(Network::Testnet),
        2 => Ok(Network::Signet),
        3 => Ok(Network::Regtest),
        _ => Err(MarketError::Network(format!("Invalid network: {}", network))),
    }
}

/// Format timestamp as human-readable string
pub fn format_timestamp(timestamp: u64) -> String {
    use chrono::{DateTime, Utc};
    let dt = DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Parse timestamp from string
pub fn parse_timestamp(timestamp_str: &str) -> Result<u64> {
    timestamp_str
        .parse::<u64>()
        .map_err(|_| MarketError::Other(format!("Invalid timestamp: {}", timestamp_str)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_market_id() {
        let id = generate_market_id();
        assert_eq!(id.len(), 8);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_sha256_hash() {
        let hash = sha256_hash("Hello, World!");
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_satoshi_btc_conversion() {
        assert_eq!(satoshi_to_btc(100_000_000), 1.0);
        assert_eq!(btc_to_satoshi(1.0), 100_000_000);
    }

    #[test]
    fn test_network_conversion() {
        assert_eq!(network_to_u8(Network::Bitcoin), 0);
        assert_eq!(network_to_u8(Network::Signet), 2);
        assert_eq!(u8_to_network(0).unwrap(), Network::Bitcoin);
        assert_eq!(u8_to_network(2).unwrap(), Network::Signet);
    }

    #[test]
    fn test_address_validation() {
        // This would need real addresses to test properly
        let invalid_addr = "invalid_address";
        assert!(!validate_address(invalid_addr, Network::Bitcoin));
    }
}