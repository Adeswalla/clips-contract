//! Metadata type definitions.
//!
//! This module contains all core metadata structures used throughout the contract.

use soroban_sdk::{contracttype, Address, String, Vec};

use crate::types::TokenId;

/// Represents an NFT attribute following the OpenSea metadata standard.
///
/// # Fields
/// - `trait_type`: The name of the trait (e.g., "virality_score", "duration")
/// - `value`: The value of the trait (e.g., "98", "42s")
///
/// # Example
/// ```rust,ignore
/// let attribute = Attribute {
///     trait_type: String::from_str(&env, "rarity"),
///     value: String::from_str(&env, "legendary"),
/// };
/// ```
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attribute {
    /// The name/type of the attribute (e.g., "Background", "Rarity")
    pub trait_type: String,
    /// The value of the attribute (e.g., "Blue", "Legendary")
    pub value: String,
}

/// Complete metadata representation for an NFT token.
///
/// This structure holds all metadata fields that can be associated with an NFT,
/// following OpenSea and general NFT metadata standards.
///
/// # Fields
/// - `metadata_uri`: Primary metadata URI (typically IPFS or Arweave)
/// - `image`: Optional image URL
/// - `animation_url`: Optional animation/video URL
/// - `description`: Optional text description
/// - `external_url`: Optional external link
/// - `attributes`: Collection of trait attributes
///
/// # Standards Compliance
/// - OpenSea Metadata Standard
/// - EIP-721 Metadata JSON Schema
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenMetadata {
    /// Primary metadata URI (IPFS, Arweave, or HTTPS)
    pub metadata_uri: String,
    /// Optional image URL
    pub image: Option<String>,
    /// Optional animation or video URL
    pub animation_url: Option<String>,
    /// Optional text description of the NFT
    pub description: Option<String>,
    /// Optional external URL for more information
    pub external_url: Option<String>,
    /// Array of attributes/traits
    pub attributes: Vec<Attribute>,
}

impl TokenMetadata {
    /// Creates a new TokenMetadata with only the required metadata_uri field.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `metadata_uri` - The primary metadata URI
    ///
    /// # Returns
    /// A new TokenMetadata instance with empty optional fields
    pub fn new(env: &soroban_sdk::Env, metadata_uri: String) -> Self {
        Self {
            metadata_uri,
            image: None,
            animation_url: None,
            description: None,
            external_url: None,
            attributes: Vec::new(env),
        }
    }

    /// Checks if any optional fields are populated.
    pub fn has_optional_fields(&self) -> bool {
        self.image.is_some()
            || self.animation_url.is_some()
            || self.description.is_some()
            || self.external_url.is_some()
            || !self.attributes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_creation() {
        // This would require a proper Soroban test environment
        // Placeholder for when proper test infrastructure is set up
    }
}
