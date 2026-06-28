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
    ClipIdMinted(u32),
    PlatformFee,
    DefaultRoyaltyBps,
    Config,
    SupportedCurrencies,
    Blacklisted(Address),
    Approval(TokenId),
    OperatorApproval(Address, Address),
    CollectionSupply(u32),
    TokenClipId(TokenId),
    ClipMinted(u32),
    Creator(TokenId),
    FrozenToken(TokenId),
    PlatformRevenue,
    RoyaltyHistory(TokenId),
    RoyaltyRecipient(TokenId),
    TokenUri(TokenId),
    WalletTokens(Address),
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
    InvalidFee = 11,
    InvalidAddress = 12,
    InvalidURI = 13,
    InvalidLimit = 14,
    UnauthorizedConfigurationUpdate = 15,
    DuplicateCurrency = 16,
    CurrencyNotFound = 17,
    InvalidConfig = 18,
    InvalidSalePrice = 19,
    RoyaltyOverflow = 20,
}
