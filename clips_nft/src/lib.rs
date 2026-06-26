//! ClipCash NFT — Soroban smart contract
//!
//! # Storage layout
//!
//! | Key              | Storage  | Type            | Description                              |
//! |------------------|----------|-----------------|------------------------------------------|
//! | `Initialized`    | instance | `bool`          | Initialization guard (set once)          |
//! | `Owner`          | instance | `Address`       | Contract owner set at initialization     |
//! | `Admins`         | instance | `Vec<Address>`  | Privileged administrator accounts        |

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, Vec};

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Contract error codes.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    /// Contract has already been initialized.
    AlreadyInitialized = 1,
    /// Operation not authorized.
    Unauthorized = 2,
    /// Duplicate admin address.
    DuplicateAdmin = 3,
}

// ---------------------------------------------------------------------------
// Storage keys
// ---------------------------------------------------------------------------

/// Storage keys used by this contract.
///
/// - `Initialized` — 1-word key; written once to prevent re-initialization.
/// - `Owner`       — 1-word key; the contract owner address.
/// - `Admins`      — 1-word key; `Vec<Address>` of administrators.
#[contracttype]
pub enum DataKey {
    /// Guard flag: present and `true` once `initialize` has run. (#440)
    Initialized,
    /// Contract owner address stored at initialization. (#441)
    Owner,
    /// List of administrator addresses. (#442)
    Admins,
}

// ---------------------------------------------------------------------------
// Guard helper  (#440)
// ---------------------------------------------------------------------------

/// Returns `true` if the contract has already been initialized.
fn is_initialized(env: &Env) -> bool {
    env.storage()
        .instance()
        .get::<DataKey, bool>(&DataKey::Initialized)
        .unwrap_or(false)
}

/// Panics with `Error::AlreadyInitialized` if the contract is already initialized.
///
/// Reuse this guard in any function that must run at most once.
fn require_not_initialized(env: &Env) -> Result<(), Error> {
    if is_initialized(env) {
        return Err(Error::AlreadyInitialized);
    }
    Ok(())
}

/// Mark the contract as initialized (write the guard flag). (#440)
fn set_initialized(env: &Env) {
    env.storage()
        .instance()
        .set(&DataKey::Initialized, &true);
}

// ---------------------------------------------------------------------------
// Owner storage helpers  (#441)
// ---------------------------------------------------------------------------

/// Persist the owner address to instance storage.
fn store_owner(env: &Env, owner: &Address) {
    env.storage().instance().set(&DataKey::Owner, owner);
}

/// Retrieve the stored owner address.
pub fn get_owner(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::Owner)
}

// ---------------------------------------------------------------------------
// Admin storage helpers  (#442)
// ---------------------------------------------------------------------------

/// Persist the admin list to instance storage.
fn store_admins(env: &Env, admins: &Vec<Address>) {
    env.storage().instance().set(&DataKey::Admins, admins);
}

/// Retrieve the stored admin list (empty vec if not set).
pub fn get_admins(env: &Env) -> Vec<Address> {
    env.storage()
        .instance()
        .get(&DataKey::Admins)
        .unwrap_or_else(|| Vec::new(env))
}

/// Add `admin` to the admin list if not already present.
///
/// Returns `Error::DuplicateAdmin` if `admin` is already in the list.
pub fn add_admin(env: &Env, admin: &Address) -> Result<(), Error> {
    let mut list = get_admins(env);
    if list.contains(admin) {
        return Err(Error::DuplicateAdmin);
    }
    list.push_back(admin.clone());
    store_admins(env, &list);
    Ok(())
}

// ---------------------------------------------------------------------------
// Contract  (#439 – initialize function)
// ---------------------------------------------------------------------------

#[contract]
pub struct ClipCashNFT;

#[contractimpl]
impl ClipCashNFT {
    /// Initialize the contract.
    ///
    /// Sets the owner and an initial list of administrators. Can only be called
    /// once; subsequent calls return `Error::AlreadyInitialized`. (#439, #440)
    ///
    /// # Arguments
    /// * `owner`  — Address that becomes the contract owner. (#441)
    /// * `admins` — Initial administrator addresses (duplicates are rejected). (#442)
    pub fn initialize(env: Env, owner: Address, admins: Vec<Address>) -> Result<(), Error> {
        // #440 — initialization guard
        require_not_initialized(&env)?;

        // Require the deployer/caller to authorize this call.
        owner.require_auth();

        // #441 — store owner
        store_owner(&env, &owner);

        // #442 — store admins (deduplicated)
        let mut deduped: Vec<Address> = Vec::new(&env);
        for admin in admins.iter() {
            if !deduped.contains(&admin) {
                deduped.push_back(admin);
            }
        }
        store_admins(&env, &deduped);

        // #440 — flip the guard flag last so storage is consistent on failure
        set_initialized(&env);

        Ok(())
    }

    /// Returns the contract owner address, if set.
    pub fn owner(env: Env) -> Option<Address> {
        get_owner(&env)
    }

    /// Returns the current list of administrators.
    pub fn admins(env: Env) -> Vec<Address> {
        get_admins(&env)
    }

    /// Returns `true` if the contract has been initialized.
    pub fn initialized(env: Env) -> bool {
        is_initialized(&env)
    }
}
