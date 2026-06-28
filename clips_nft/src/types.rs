use soroban_sdk::{contracterror, contracttype, Address, String};

pub type TokenId = u32;

#[contracttype]
#[derive(Clone)]
pub struct TokenData {
    pub owner: Address,
    pub clip_id: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Royalty {
    pub recipient: Address,
    pub basis_points: u32,
    pub asset_address: Option<Address>,
}

#[contracttype]
#[derive(Clone)]
pub struct RoyaltyInfo {
    pub receiver: Address,
    pub royalty_amount: i128,
    pub asset_address: Option<Address>,
}

#[contracttype]
#[derive(Clone)]
pub struct RoyaltyPayment {
    pub token_id: TokenId,
    pub recipient: Address,
    pub amount: i128,
    pub timestamp: u64,
}

/// Minimal contract-wide config stored by the storage sub-module.
#[contracttype]
#[derive(Clone)]
pub struct Config {
    pub admin: Address,
    pub max_royalty_bps: u32,
    pub mint_cooldown_secs: u64,
    pub platform_fee_bps: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct MintEvent {
    pub to: Address,
    pub clip_id: u32,
    pub token_id: TokenId,
    pub metadata_uri: String,
}

#[contracttype]
#[derive(Clone)]
pub struct BurnEvent {
    pub owner: Address,
    pub token_id: TokenId,
}

#[contracttype]
pub enum DataKey {
    Admin,
    NextTokenId,
    Paused,
    Signer,
    Token(TokenId),
    Metadata(TokenId),
    Royalty(TokenId),
    /// Maps clip_id → token_id; also used as existence marker for a minted clip.
    ClipIdMinted(u32),
    PlatformFee,
    DefaultRoyaltyBps,
    Config,
    /// List of supported payment currency addresses.
    SupportedCurrencies,
    /// Blacklisted wallet address.
    Blacklisted(Address),
    /// Single-token approval: address approved to transfer token_id.
    Approval(TokenId),
    /// Operator approval: (owner, operator) → approved.
    OperatorApproval(Address, Address),
    /// Minted supply counter per collection.
    CollectionSupply(u32),
    /// Maps token_id → clip_id (reverse of ClipIdMinted).
    TokenClipId(TokenId),
    /// Existence marker for the minted-clip index (bool).
    ClipMinted(u32),
    /// Creator wallet for a token.
    Creator(TokenId),
    /// Whether a token is frozen (persistent).
    FrozenToken(TokenId),
    /// Cumulative platform revenue in the smallest unit.
    PlatformRevenue,
    /// Royalty payment history for a token.
    RoyaltyHistory(TokenId),
    /// Royalty recipient address for a token.
    RoyaltyRecipient(TokenId),
    /// Custom metadata URI override for a token.
    TokenUri(TokenId),
    /// All token IDs owned by a wallet.
    WalletTokens(Address),
    /// Per-event-type emission counter (event_type_id → count).
    EventCounter(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    ContractPaused = 4,
    NotPaused = 5,
    TokenNotFound = 6,
    ClipAlreadyMinted = 7,
    SignerNotSet = 8,
    InvalidSignature = 9,
    InvalidBasisPoints = 10,
    /// Fee value is outside the allowed range.
    InvalidFee = 11,
    /// Address is invalid or empty.
    InvalidAddress = 12,
    /// Metadata URI is empty or malformed.
    InvalidURI = 13,
    /// Collection limit is zero or exceeds the maximum.
    InvalidLimit = 14,
    /// Caller is not authorized to update configuration.
    UnauthorizedConfigurationUpdate = 15,
    /// Currency already exists in the supported list.
    DuplicateCurrency = 16,
    /// Currency not found in the supported list.
    CurrencyNotFound = 17,
    /// Config values are out of range or structurally invalid.
    InvalidConfig = 18,
    /// Sale price must be positive.
    InvalidSalePrice = 19,
    /// Royalty amount calculation overflowed.
    RoyaltyOverflow = 20,
}
