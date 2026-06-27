use soroban_sdk::{contracttype, Address, String};

pub type TokenId = u32;

/// Packed token ownership data stored per token.
#[contracttype]
#[derive(Clone)]
pub struct TokenData {
    pub owner: Address,
    pub clip_id: u32,
}

/// Royalty configuration for a token.
#[contracttype]
#[derive(Clone)]
pub struct Royalty {
    pub recipient: Address,
    /// 0 – 10_000 (basis points, i.e. 0%–100%)
    pub basis_points: u32,
    pub asset_address: Option<Address>,
}

/// Royalty payment breakdown returned by `royalty_info`.
#[contracttype]
#[derive(Clone)]
pub struct RoyaltyInfo {
    pub receiver: Address,
    pub royalty_amount: i128,
    pub asset_address: Option<Address>,
}

/// Minted-event data.
#[contracttype]
#[derive(Clone)]
pub struct MintEvent {
    pub to: Address,
    pub clip_id: u32,
    pub token_id: TokenId,
    pub metadata_uri: String,
}

/// Global contract configuration.
///
/// Stored once in instance storage under [`StorageKey::Config`].
#[contracttype]
#[derive(Clone)]
pub struct Config {
    pub admin: Address,
    /// Maximum royalty in basis points (default 10_000 = 100%).
    pub max_royalty_bps: u32,
    /// Minimum seconds between mints per address (0 = no cooldown).
    pub mint_cooldown_secs: u64,
    /// Platform fee in basis points applied on top of creator royalty.
    pub platform_fee_bps: u32,
}
