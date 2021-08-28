//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! [get it's current value][get_num] or [reset].
//!
//! [increment]: struct.Counter.html#method.increment
//! [decrement]: struct.Counter.html#method.decrement
//! [get_num]: struct.Counter.html#method.get_num
//! [reset]: struct.Counter.html#method.reset

use std::collections::HashMap;
use std::fmt::Debug;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{PanicOnDefault, Promise, env, log, near_bindgen};
use near_sdk::{AccountId};
use near_sdk::json_types::{U128, U64};
use chrono::prelude::*;

use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct Stake {
    pub amount: U128,
    pub time: U64,
    pub profit: U128,
}



#[near_bindgen]
impl Contract {

    /*
    stake near to pool
    */
    #[payable]
    pub fn stake(&mut self, amount: U128) {                        
        let sender = env::predecessor_account_id();
        let user = self.users.entry(sender.to_string()).or_insert(new_user());
        let amount = u128::from(amount);
        assert!(amount > 0, "not enough amount!");
        assert!(amount <= env::attached_deposit(), "not enough balance!");
        let now = env::block_timestamp();
        self.stake_amount = self.stake_amount + amount;
        if user.stakes.len() == 0 {
            self.stake_users.push(sender.clone());
        }
        user.stakes.push(Stake {
            amount: U128::from(amount),
            time: U64::from(now),
            profit: U128::from(0),
        });
        self.max_amount_allowed = ((self.stake_amount + self.profit_amount) as f64 * self.amount_allowed_rate as f64) as u128
    }

    /*
    unstake from pool
    */
    pub fn unstake(&mut self, amount: U128, index: usize) {
        let sender = env::predecessor_account_id();
        let user = self.users.get_mut(&sender).unwrap();
        let amount = u128::from(amount);
        let stake = user.stakes.get_mut(index).unwrap();
        let now = env::block_timestamp();
        
        assert!(u64::from(stake.time) < u64::from(env::block_timestamp()), "in lock period");
        assert!(amount <= u128::from(stake.amount.clone()), "not enough balance");
        assert!(amount > 0, "not enough amount!");
        assert!(amount <= env::account_balance(), "not enough balance!");
        assert!(u128::from(stake.amount.clone()) >= amount, "not enough balance");

        Promise::new(sender.to_string()).transfer(amount.clone());
        self.stake_amount = self.stake_amount - amount;
        stake.amount = U128::from(u128::from(stake.amount) - amount);
        stake.time = U64::from(now);
        if stake.amount == U128::from(0) {
            user.stakes.remove(index);
        }
        if user.stakes.len() == 0 {
            let index = self.stake_users.iter().position(|x| *x == sender).unwrap();
            self.stake_users.remove(index);
        }
        self.max_amount_allowed = ((self.stake_amount + self.profit_amount) as f64 * self.amount_allowed_rate as f64) as u128
    }

    /*
    profits doesn't compund to stakes, it needs to harvest manually
    */
    pub fn harvest(&mut self, index: usize) {
        let sender = env::predecessor_account_id();
        let user = self.users.get_mut(&sender).unwrap();
        let stake = user.stakes.get_mut(index).unwrap();
        let amount = u128::from(stake.profit);
        assert!(amount <= env::account_balance(), "not enough balance!");
        Promise::new(sender.to_string()).transfer(amount.clone());
        self.profit_amount -= amount;
        stake.profit = U128::from(0);
        self.max_amount_allowed = ((self.stake_amount + self.profit_amount) as f64 * self.amount_allowed_rate as f64) as u128
    }

    pub fn treasury(&mut self) {
        assert!(self.last_treasury_time < env::block_timestamp(), "too quick for treasury");
        assert!(self.treasury_amount > self.treasury_threshold, "not enough treasury");
        let gamers_amount = self.treasury_amount as f64 * self.treasury_shares[0] as f64;
        let stakers_amount = self.treasury_amount as f64 * self.treasury_shares[1] as f64;
        let team_amount = self.treasury_amount as f64 * self.treasury_shares[2] as f64;
        let mut users_count = 0;
        for (_, user) in self.users.iter() {
            if user.history_bets.len() == 0 {
                continue;
            }
            let last_bet = user.history_bets.get(user.history_bets.len() - 1).unwrap();
            if u64::from(last_bet.time) > self.last_treasury_time {
                users_count += 1
            }
        }
        let each_gamer_amount = (gamers_amount as f64 / users_count as f64) as u128;
        for (_, user) in self.users.iter_mut() {
            if user.history_bets.len() > 0 {
                user.balance = U128::from(u128::from(user.balance) + each_gamer_amount);
            }
            for stake in user.stakes.iter_mut() {
                let time_delta = env::block_timestamp() - u64::from(stake.time);
                let mut time_rate = 1.0;
                for (i, time)in self.step_time.iter().enumerate() {
                    if time_delta > *time as u64 {
                        time_rate = 1 as f32 + self.step_rate[i];
                        break;
                    }
                }
                let rate = u128::from(stake.amount) as f64 * time_rate as f64 / u128::from(self.stake_amount) as f64;
                let profit = (stakers_amount as f64 * rate) as u128;
                stake.profit = U128::from(u128::from(stake.profit) + profit);
            }
        }

        Promise::new(self.creator.clone()).transfer(team_amount as u128);

        self.treasury_amount = 0;
        self.last_treasury_time = env::block_timestamp();
    }

}

impl Contract {
    /*
    profit needs to calculate every round
    */
    pub(crate) fn cal_profit(&mut self, total_bet: u128, total_win:u128) {
        if u128::from(self.profit_amount) + total_bet > total_win {
            self.profit_amount = self.profit_amount + total_bet - total_win;  // deal with total profit
        } else {
            let delta = total_win - total_bet - u128::from(self.profit_amount);  // if pool lose, decrease pool staking amount
            self.profit_amount = 0;
            self.stake_amount = self.stake_amount - delta;
        }
        
        let mut delta = 0;
        if total_bet < total_win {
            delta = total_win - total_bet;
        }
        else {
            delta = total_bet - total_win;
            let treasury_amount = (delta as f64 * self.treasury_rate as f64) as u128;   // deal with treasury
            self.treasury_amount = self.treasury_amount + treasury_amount;
            delta = delta - treasury_amount;
        }

        let now = env::block_timestamp();
        let new_user = &mut new_user();
        for key in self.stake_users.iter() {                                           // deal with each stake
            let user: &mut User = self.users.get_mut(key).unwrap_or(new_user);
            for stake in user.stakes.iter_mut() {
                let time_delta = now - u64::from(stake.time);
                let mut time_rate = 1.0;
                for (i, time)in self.step_time.iter().enumerate() {
                    if time_delta > *time as u64 {
                        time_rate = 1 as f32 + self.step_rate[i];
                        break;
                    }
                }
                let rate = u128::from(stake.amount) as f64 * time_rate as f64 / u128::from(self.stake_amount) as f64;
                let delta_with_rate = (delta as f64 * rate) as u128;
                if total_bet < total_win {                                                  //when pool lose
                    if u128::from(stake.profit) > delta_with_rate {
                        stake.profit = U128::from(u128::from(stake.profit) - delta_with_rate);  
                    } else {
                        delta = delta_with_rate - u128::from(stake.profit);
                        stake.profit = U128::from(0);
                        stake.amount = U128::from(u128::from(stake.amount) - delta);
                    }
                }
                else {
                    stake.profit = U128::from(u128::from(stake.profit) + delta_with_rate);   // add profit directly
                }
                
            }
        }
        self.max_amount_allowed = ((self.stake_amount + self.profit_amount) as f64 * self.amount_allowed_rate as f64) as u128   //re-calculate max amount for bets
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::testing_env;
    use near_sdk::VMContext;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;

    // simple helper function to take a string literal and return a ValidAccountId
    fn to_valid_account(account: &str) -> ValidAccountId {
        ValidAccountId::try_from(account.to_string()).expect("Invalid account")
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context() -> VMContext {
        VMContext {
            current_account_id: "alice".to_string(),
            signer_account_id: "bob".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "bob".to_string(),
            input: vec![],
            block_index: 0,
            block_timestamp: env::block_timestamp() - 100,
            account_balance: 100000000000000000,
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 10000000000000000,
            prepaid_gas: 10u64.pow(15),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired

    #[test]
    fn cal_profit() {
        let context = get_context();
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Contract::new();
        contract.cal_profit(123, 123);
    }

    #[test]
    fn stake() {
        let context = get_context();
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Contract::new();
        contract.stake(U128::from(10000000000000000));
    }
}
