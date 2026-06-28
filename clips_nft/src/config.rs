//! Global configuration storage module (issue #457).
//!
//! Centralises all contract-level config values behind typed getters and
//! setters backed by `instance` persistent storage.  Every value written here
//! survives contract upgrades unchanged because instance storage travels with
//! the contract address.
//!
//! ## Storage layout
//!
//! | `DataKey`                      | Type     | Default | Description                          |
//! |-------------------------------|----------|---------|--------------------------------------|
//! | `PlatformFeeBps`              | `u32`    | `100`   | Platform royalty in basis points     |
//! | `DefaultRoyaltyBps`           | `u32`    | `0`     | Default creator royalty in bps       |
//! | `MintCooldownSeconds`         | `u64`    | `0`     | Per-wallet mint cooldown             |
//! | `CircuitBreakerEnabled`       | `bool`   | `false` | Whether the circuit breaker is on    |
//! | `CircuitBreakerThreshold`     | `u64`    | `100`   | Max mints per window before trip     |
//! | `CircuitBreakerWindowSeconds` | `u64`    | `60`    | Rolling window length in seconds     |

use soroban_sdk::{contracttype, Env};

use crate::DataKey;

/// Snapshot of all global configuration values.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalConfig {
    pub platform_fee_bps: u32,
    pub default_royalty_bps: u32,
    pub mint_cooldown_seconds: u64,
    pub circuit_breaker_enabled: bool,
    pub circuit_breaker_threshold: u64,
    pub circuit_breaker_window_seconds: u64,
}

const DEFAULT_PLATFORM_FEE_BPS: u32 = 100;
const DEFAULT_ROYALTY_BPS: u32 = 0;
const DEFAULT_COOLDOWN: u64 = 0;
const DEFAULT_CB_ENABLED: bool = false;
const DEFAULT_CB_THRESHOLD: u64 = 100;
const DEFAULT_CB_WINDOW: u64 = 60;

/// Read the current global configuration from storage.
pub fn get(env: &Env) -> GlobalConfig {
    GlobalConfig {
        platform_fee_bps: env
            .storage()
            .instance()
            .get(&DataKey::PlatformFeeBps)
            .unwrap_or(DEFAULT_PLATFORM_FEE_BPS),
        default_royalty_bps: env
            .storage()
            .instance()
            .get(&DataKey::DefaultRoyaltyBps)
            .unwrap_or(DEFAULT_ROYALTY_BPS),
        mint_cooldown_seconds: env
            .storage()
            .instance()
            .get(&DataKey::MintCooldownSeconds)
            .unwrap_or(DEFAULT_COOLDOWN),
        circuit_breaker_enabled: env
            .storage()
            .instance()
            .get(&DataKey::CircuitBreakerEnabled)
            .unwrap_or(DEFAULT_CB_ENABLED),
        circuit_breaker_threshold: env
            .storage()
            .instance()
            .get(&DataKey::CircuitBreakerThreshold)
            .unwrap_or(DEFAULT_CB_THRESHOLD),
        circuit_breaker_window_seconds: env
            .storage()
            .instance()
            .get(&DataKey::CircuitBreakerWindowSeconds)
            .unwrap_or(DEFAULT_CB_WINDOW),
    }
}

/// Persist a full `GlobalConfig` snapshot to storage.
///
/// Individual fields are written separately so that a partial update (e.g.
/// only `platform_fee_bps` changed) still uses the existing setters on the
/// contract and the values remain consistent.
pub fn set(env: &Env, cfg: &GlobalConfig) {
    env.storage()
        .instance()
        .set(&DataKey::PlatformFeeBps, &cfg.platform_fee_bps);
    env.storage()
        .instance()
        .set(&DataKey::DefaultRoyaltyBps, &cfg.default_royalty_bps);
    env.storage()
        .instance()
        .set(&DataKey::MintCooldownSeconds, &cfg.mint_cooldown_seconds);
    env.storage()
        .instance()
        .set(&DataKey::CircuitBreakerEnabled, &cfg.circuit_breaker_enabled);
    env.storage()
        .instance()
        .set(&DataKey::CircuitBreakerThreshold, &cfg.circuit_breaker_threshold);
    env.storage()
        .instance()
        .set(&DataKey::CircuitBreakerWindowSeconds, &cfg.circuit_breaker_window_seconds);
}
