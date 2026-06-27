#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

pub mod errors;
pub mod storage;
pub mod types;

#[cfg(test)]
mod tests;

use errors::Error;
use storage::{get_config, set_config, validate_config};
use types::Config;

#[contract]
pub struct ClipCashNFT;

#[contractimpl]
impl ClipCashNFT {
    /// Initialise the contract. May only be called once.
    pub fn init(env: Env, admin: soroban_sdk::Address) {
        if env
            .storage()
            .instance()
            .has(&storage::StorageKey::Config)
        {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }
        let config = Config {
            admin,
            max_royalty_bps: 10_000,
            mint_cooldown_secs: 0,
            platform_fee_bps: 0,
        };
        set_config(&env, &config);
    }

    // ── Config helpers ────────────────────────────────────────────────────

    /// Return the current global config.
    pub fn get_config(env: Env) -> Config {
        get_config(&env)
    }

    /// Update config fields. Admin-only.
    pub fn set_config(env: Env, admin: soroban_sdk::Address, new_config: Config) -> Result<(), Error> {
        admin.require_auth();
        let cfg = get_config(&env);
        if cfg.admin != admin {
            return Err(Error::Unauthorized);
        }
        validate_config(&new_config)?;
        set_config(&env, &new_config);
        Ok(())
    }
}
