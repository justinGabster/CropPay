#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct CropPay;

#[contractimpl]
impl CropPay {
    pub fn hello(_env: Env) -> u32 {
        1
    }
}