#![cfg(test)]

use soroban_sdk::{Env, Address, testutils::Address as TestAddress};
use crate::CropPayContract;

#[test]
fn test_happy_path() {
    let env = Env::default();
    let farmer = TestAddress::random(&env);
    CropPayContract::record_delivery(env.clone(), farmer.clone(), 100);
    CropPayContract::redeem(env.clone(), farmer.clone(), 50);
    assert_eq!(CropPayContract::balance(env.clone(), farmer.clone()), 50);
}

#[test]
#[should_panic(expected = "Insufficient balance")]
fn test_insufficient_balance() {
    let env = Env::default();
    let farmer = TestAddress::random(&env);
    CropPayContract::record_delivery(env.clone(), farmer.clone(), 30);
    CropPayContract::redeem(env.clone(), farmer.clone(), 50);
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    let farmer = TestAddress::random(&env);
    CropPayContract::record_delivery(env.clone(), farmer.clone(), 200);
    assert_eq!(CropPayContract::balance(env.clone(), farmer.clone()), 200);
}

#[test]
fn test_multiple_deliveries() {
    let env = Env::default();
    let farmer = TestAddress::random(&env);
    CropPayContract::record_delivery(env.clone(), farmer.clone(), 100);
    CropPayContract::record_delivery(env.clone(), farmer.clone(), 150);
    assert_eq!(CropPayContract::balance(env.clone(), farmer.clone()), 250);
}

#[test]
fn test_redeem_exact_balance() {
    let env = Env::default();
    let farmer = TestAddress::random(&env);
    CropPayContract::record_delivery(env.clone(), farmer.clone(), 80);
    CropPayContract::redeem(env.clone(), farmer.clone(), 80);
    assert_eq!(CropPayContract::balance(env.clone(), farmer.clone()), 0);
}
