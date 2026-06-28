#![no_std]

mod blacklist;
mod clip_id_storage;
mod collection_supply;
mod config;
mod config_guard;
mod config_validator;
mod creator_storage;
mod default_royalty;
mod errors;
mod frozen_token;
mod minted_clip_index;
mod operator_approval;
mod pause_state;
mod payment_currency;
mod platform_fee;
mod platform_revenue;
mod royalty_history;
mod royalty_recipient;
mod royalty_storage;
mod safe_math;
pub mod storage;
mod token_approval;
mod token_metadata_storage;
mod token_storage;
mod token_uri_storage;
mod types;
mod wallet_token_index;

mod storage_cleanup;

#[cfg(test)]
mod tests;

pub use types::{DataKey, Error};

use soroban_sdk::{contract, contractimpl, Address, Env};
use types::Config;

#[contract]
pub struct ClipCashNFT;

#[contractimpl]
impl ClipCashNFT {
    /// Initialize the contract, setting the admin and default config.
    ///
    /// Panics if already initialized.
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        let default_cfg = Config {
            admin: admin.clone(),
            max_royalty_bps: 10_000,
            mint_cooldown_secs: 0,
            platform_fee_bps: 0,
        };
        storage::config::set_config(&env, &default_cfg);
    }

    /// Return the current contract config.
    pub fn get_config(env: Env) -> Config {
        storage::config::get_config(&env)
    }

    /// Update contract config. Admin only; validates basis-point ranges.
    pub fn set_config(env: Env, admin: Address, cfg: Config) -> Result<(), Error> {
        config_guard::require_config_admin(&env, &admin)?;
        storage::config::validate_config(&cfg)?;
        storage::config::set_config(&env, &cfg);
        Ok(())
    }
}
