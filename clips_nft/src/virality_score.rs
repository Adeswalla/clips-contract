use soroban_sdk::{contracttype, Env};

use crate::types::{DataKey, Error, TokenId};

/// Valid range: 0–10_000 (0.00 % – 100.00 %, two-decimal precision)
pub const MAX_VIRALITY_SCORE: u32 = 10_000;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViralityScore {
    pub score: u32,
}

pub fn set_virality_score(env: &Env, token_id: TokenId, score: u32) -> Result<(), Error> {
    if score > MAX_VIRALITY_SCORE {
        return Err(Error::InvalidBasisPoints);
    }
    env.storage()
        .persistent()
        .set(&DataKey::ViralityScore(token_id), &ViralityScore { score });
    Ok(())
}

pub fn get_virality_score(env: &Env, token_id: TokenId) -> Result<ViralityScore, Error> {
    env.storage()
        .persistent()
        .get(&DataKey::ViralityScore(token_id))
        .ok_or(Error::TokenNotFound)
}
