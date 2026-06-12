#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
pub enum DataKey {
    Counter,
}

#[contract]
pub struct VisitCounter;

#[contractimpl]
impl VisitCounter {
    pub fn increment(env: Env) -> u32 {
        let count: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        let new_count = count + 1;

        env.storage()
            .persistent()
            .set(&DataKey::Counter, &new_count);

        new_count
    }

    pub fn get_count(env: Env) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Counter)
            .unwrap_or(0)
    }
}