use soroban_sdk::{contractimpl, Env, Symbol, Address, BytesN};

pub struct CropPayContract;

// Storage keys
const KEY_BALANCE: &str = "balance";

#[contractimpl]
impl CropPayContract {
    // Record delivery and mint cooperative tokens
    pub fn record_delivery(env: Env, farmer: Address, amount: i128) {
        // Increase farmer balance
        let key = (KEY_BALANCE, farmer.clone());
        let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(current + amount));
    }

    // Redeem cooperative tokens for USDC
    pub fn redeem(env: Env, farmer: Address, amount: i128) {
        let key = (KEY_BALANCE, farmer.clone());
        let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);

        if current < amount {
            panic!("Insufficient balance");
        }

        env.storage().persistent().set(&key, &(current - amount));

        // Emit event (simulating USDC transfer)
        env.events().publish((Symbol::short("redeem"), farmer), amount);
    }

    // Check balance
    pub fn balance(env: Env, farmer: Address) -> i128 {
        let key = (KEY_BALANCE, farmer);
        env.storage().persistent().get(&key).unwrap_or(0)
    }
}
