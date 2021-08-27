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
use std::num;
use crate::*;



pub fn check_win(number:u8, b: &Bet) -> bool {
    let mut won = false;
    if number == 0 {
        won = b.bet_type == 5 && b.number == 0;                   /* bet on 0 */
    } else {
    if b.bet_type == 5 { 
        won = b.number == number;                              /* bet on number */
    } else if b.bet_type == 4 {
        if b.number == 0 { won = number % 2 == 0; }              /* bet on even */
        if b.number == 1 { won = number % 2 == 1; }              /* bet on odd */
    } else if b.bet_type == 3 {            
        if b.number == 0 { won = number <= 18; }                /* bet on low 18s */
        if b.number == 1 { won = number >= 19; }                 /* bet on high 18s */
    } else if b.bet_type == 2 {                               
        if b.number == 0 { won = number <= 12; }                 /* bet on 1st dozen */
        if b.number == 1 { won = number > 12 && number <= 24; }  /* bet on 2nd dozen */
        if b.number == 2 { won = number > 24; }                  /* bet on 3rd dozen */
    } else if b.bet_type == 1 {               
        if b.number == 0 { won = number % 3 == 1; }              /* bet on left column */
        if b.number == 1 { won = number % 3 == 2; }              /* bet on middle column */
        if b.number == 2 { won = number % 3 == 0; }              /* bet on right column */
    } else if b.bet_type == 0 {
        if b.number == 0 {                                     /* bet on black */
        if number <= 10 || number >= 20 && number <= 28 {
            won = number % 2 == 0;
        } else {
            won = number % 2 == 1;
        }
        } else {                                                 /* bet on red */
        if number <= 10 || number >= 20 && number <= 28 {
            won = number % 2 == 1;
        } else {
            won = number % 2 == 0;
        }
        }
    }
    }
    won
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct Bet {
    pub bet_type: u8,
    pub number: u8,
    pub chips: U128,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct HistoryBet {
    pub bet_type: u8,
    pub number: u8,
    pub chips: U128,
    pub win_chips: U128,
    pub win_number: u8,
}



#[near_bindgen]
impl Contract {

    #[payable]
    pub fn bet(&mut self, bets: Vec<Bet>) {
        let sender = env::predecessor_account_id();
        let user = self.users.entry(sender.to_string()).or_insert(new_user());
        assert!(bets.len() > 0, "you have 0 bets");
        let mut total:u128= 0;
        for item in bets.iter() {
            total += u128::from(item.chips);
            assert!(item.bet_type <= 5);
            assert!(item.number <= self.number_range[item.bet_type as usize]); 
        }
        let balance = u128::from(user.balance) + env::attached_deposit();
        assert!(balance >= total, "not enough balance");
        let bet_amount = self.bet_amount + total;
        assert!(bet_amount < self.max_amount_allowed, "exceed max bet amount allowed");

        self.bet_amount = bet_amount;
        user.balance = U128::from(balance - total);
        if user.bets.len() == 0 {
            self.bet_users.push(sender);
        }
        user.bets = [user.bets.clone(), bets.clone()].concat();
    }


    
    pub fn spin_wheel(&mut self) {
        /* are we allowed to spin the wheel? */
        assert!(env::block_index() > self.round_block_index + self.round_delta, "too quick to spin");
        let sender = env::predecessor_account_id();
        let creator = env::current_account_id();
        // assert!(sender == creator, "not contract owner");
        assert!(self.bet_users.len() > 0, "no bets");

        /* are there any bets? */
        /* next time we are allowed to spin the wheel again */
        
        /* calculate 'random' number */
        let hash = env::sha256(&env::random_seed());
        let mut hash_bytes: [u8;4] = [0;4];
        for i in 0..4 {
            hash_bytes[i] = hash.get(i).unwrap().clone();
        }
        let hash_number = u32::from_be_bytes(hash_bytes);
        let number: u8 = hash_number as u8 % 37 ;
        /* check every bet for this number */
        
        let mut total_bet:u128 = 0;
        let mut total_win:u128 = 0;

        for player_str in self.bet_users.iter() {
            let mut user = self.users.get_mut(player_str).unwrap();
            let mut bet_amount = 0;
            let mut win_amount = 0;
            for b in user.bets.iter() {
                let won = check_win(number, b);
                bet_amount += u128::from(b.chips);
                /* if winning bet, add to player balance balance */
                if won {
                    let win_chips = self.payouts[b.bet_type as usize] as u128 * u128::from(b.chips);
                    win_amount += win_chips;
                    user.history_bets.push(HistoryBet{
                        bet_type: b.bet_type,
                        number: b.number,
                        chips: b.chips,
                        win_chips: U128::from(win_chips),
                        win_number: number,
                    });
                } else {
                    user.history_bets.push(HistoryBet{
                        bet_type: b.bet_type,
                        number: b.number,
                        chips: b.chips,
                        win_chips: U128::from(0),
                        win_number: number,
                    });
                }
            }
            user.balance = U128::from(u128::from(user.balance) + win_amount);
            user.bets.clear();
            total_bet += bet_amount;
            total_win += win_amount;
        }

        self.cal_profit(total_bet, total_win);
        
        self.bet_users.clear();
        self.bet_amount = 0;
        self.round_block_index = env::block_index();
        self.round_index += 1;
        self.win_number = number;
    }

    #[payable]
    pub fn deposit(&mut self, amount: U128) {
        let sender = env::predecessor_account_id();
        let user = self.users.entry(sender.to_string()).or_insert(new_user());
        let amount = u128::from(amount);
        assert!(amount > 0, "not enough amount!");
        assert!(amount <= env::attached_deposit(), "not enough balance!");
        user.balance = U128::from(u128::from(user.balance) + amount);
    }

    pub fn withdraw(&mut self, amount: U128) {
        let sender = env::predecessor_account_id();
        let user = self.users.get_mut(&sender).unwrap();
        let amount = u128::from(amount);
        assert!(amount <= u128::from(user.balance), "not enough balance");
        assert!(amount > 0, "not enough amount!");
        assert!(amount <= env::account_balance(), "not enough balance!");
        Promise::new(sender.to_string()).transfer(amount.clone());
        user.balance = U128::from(u128::from(user.balance) - amount);
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
    fn spin_wheel() {
        // set up the mock context into the testing environment
        let context = get_context();
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Contract::new();
        let bets: Vec<Bet> = vec![Bet {
            bet_type: 5,
            number: 0,
            chips: U128::from(10000000000000000)
        }];
        
        // println!("{}", contract.get_status());
        // let bets = 
        // //println!("Value after increment: {}", U128::from(contract.get_status().balance));
        // // confirm that we received 1 when calling get_num
        let number = 0;//contract.spin_wheel(bets);
        println!("{}", number);
    }
}
