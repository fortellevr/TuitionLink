#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, Map};

#[contract]
pub struct TuitionLink;

// Storage key
const PAYMENTS: Symbol = symbol_short!("PAY");

#[contractimpl]
impl TuitionLink {

    // Pay tuition and store receipt
    pub fn pay_tuition(env: Env, student: Address, amount: i128) {
        student.require_auth();

        let mut payments: Map<Address, i128> =
            env.storage().instance().get(&PAYMENTS).unwrap_or(Map::new(&env));

        payments.set(student.clone(), amount);

        env.storage().instance().set(&PAYMENTS, &payments);
    }

    // Check payment
    pub fn get_payment(env: Env, student: Address) -> i128 {
        let payments: Map<Address, i128> =
            env.storage().instance().get(&PAYMENTS).unwrap_or(Map::new(&env));

        payments.get(student).unwrap_or(0)
    }
}