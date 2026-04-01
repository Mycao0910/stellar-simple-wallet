#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, Map
};

#[contract]
pub struct WalletContract;

#[contractimpl]
impl WalletContract {

    // 💰 Deposit
    pub fn deposit(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let key_balances = symbol_short!("BAL");

        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&key_balances).unwrap_or(Map::new(&env));

        let current = balances.get(user.clone()).unwrap_or(0);

        balances.set(user.clone(), current + amount);

        env.storage().instance().set(&key_balances, &balances);
    }

    // 💸 Withdraw
    pub fn withdraw(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let key_balances = symbol_short!("BAL");

        let mut balances: Map<Address, i128> =
            env.storage().instance().get(&key_balances).unwrap_or(Map::new(&env));

        let current = balances.get(user.clone()).unwrap_or(0);

        if current >= amount {
            balances.set(user.clone(), current - amount);
        } else {
            panic!("Not enough balance");
        }

        env.storage().instance().set(&key_balances, &balances);
    }

    // 📊 Get balance
    pub fn get_balance(env: Env, user: Address) -> i128 {
        let key_balances = symbol_short!("BAL");

        let balances: Map<Address, i128> =
            env.storage().instance().get(&key_balances).unwrap_or(Map::new(&env));

        balances.get(user).unwrap_or(0)
    }
}
