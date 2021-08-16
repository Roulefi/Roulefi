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
use near_sdk::{PanicOnDefault, Promise, env, near_bindgen};
use near_sdk::{AccountId};
use near_sdk::json_types::{U128, U64};

near_sdk::setup_alloc!();


fn check_win(number:u8, b: &Bet) -> bool {
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


#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Roulette {

     /*
      bet_types are as follow:
        0: color
        1: column
        2: dozen
        3: eighteen
        4: modulus
        5: number
        
      Depending on the bet_type, number will be:
        color: 0 for black, 1 for red
        column: 0 for left, 1 for middle, 2 for right
        dozen: 0 for first, 1 for second, 2 for third
        eighteen: 0 for low, 1 for high
        modulus: 0 for even, 1 for odd
        number: number
    */
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    max_amount_allowed: u128,
    number_range: [u8; 6],
    payouts: [u8; 6],

    next_round_timestamp: u64,
    necessary_balance: u128,
    users: HashMap<AccountId, User>
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Status {
    pub next_round_timestamp: U64,
    pub balance: U128,
    pub user: User
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct User {
    pub balance: U128,
    pub history_bets: Vec<HistoryBet>
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
impl Roulette {
    #[init]
    pub fn new() -> Self {
        // assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            necessary_balance: 0,
            next_round_timestamp: env::block_timestamp() as u64,
            payouts: [2,3,3,2,2,36],
            number_range: [1,2,2,1,1,36],
            // bet_amount: 10000000000000000000000, /* 0.01 ether */
            max_amount_allowed: 2000000000000000000000000, /* 2 ether */
            users: HashMap::new()
        }
    }
    
    pub fn get_status(&self, sender: AccountId) -> Status {
        let user = self.users.get(&sender);
        let user = match user {
            Some(v) => v.clone(),
            None => User {
                balance: U128::from(0),
                history_bets: Vec::new()
            } 
        };
        let status = Status {
            next_round_timestamp: self.next_round_timestamp.into(),      // when can we play again
            balance: env::account_balance().into(),
            user: user
        };
        status
    }

    #[payable]
    pub fn spin_wheel(&mut self, bets: Vec<Bet>) -> u8 {
        /* are we allowed to spin the wheel? */
        assert!(env::block_timestamp() > self.next_round_timestamp, "too quick to spin");
        let sender = env::predecessor_account_id();
        let user = self.users.entry(sender.to_string()).or_insert(User {
            balance: U128::from(0),
            history_bets: Vec::new()
        });
        assert!(bets.len() > 0, "you have 0 bets");
        let mut total:u128= 0;
        for item in bets.iter() {
            total += u128::from(item.chips);
            assert!(item.bet_type <= 5);
            assert!(item.number <= self.number_range[item.bet_type as usize]); 
        }
        let balance = u128::from(user.balance) + env::attached_deposit();
        assert!(u128::from(balance) >= total, "not enough balance");
        user.balance = U128::from(balance - total);

        /* are there any bets? */
        /* next time we are allowed to spin the wheel again */
        self.next_round_timestamp = env::block_timestamp() as u64;
        /* calculate 'random' number */
        let hash = env::sha256(&env::random_seed());
        let mut hash_bytes: [u8;4] = [0;4];
        for i in 0..4 {
            hash_bytes[i] = hash.get(i).unwrap().clone();
        }
        let hash_number = u32::from_be_bytes(hash_bytes);
        let number: u8 = hash_number as u8 % 37 ;
        /* check every bet for this number */
        
        for b in bets.iter() {
            let won = check_win(number, b);
            /* if winning bet, add to player balance balance */
            if won {
                let win_chips = self.payouts[b.bet_type as usize] as u128 * u128::from(b.chips);
                user.balance = U128::from(u128::from(user.balance) + win_chips);
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
        /* reset necessary_balance */
        // self.necessary_balance = 0;
        /* check if to much money in the bank */
        // if env::account_balance() > self.max_amount_allowed {
        //     //self.take_profits();
        // }
        number
        /* returns 'random' number to UI */
        // emit RandomNumber(number);
    }

    #[payable]
    pub fn deposit(&mut self, amount: U128) {
        let sender = env::predecessor_account_id();
        let user = self.users.entry(sender.to_string()).or_insert(User {
            balance: U128::from(0),
            history_bets: Vec::new()
        });
        assert!(u128::from(amount) > 0, "not enough amount!");
        assert!(u128::from(amount) <= env::attached_deposit(), "not enough balance!");
        user.balance = U128::from(u128::from(user.balance) + u128::from(amount));
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
      
    pub fn take_profits(&self) {
        assert_eq!(
            env::current_account_id(),
            env::predecessor_account_id(),
            "Can only be called by owner"
        );
        let amount: u128 = env::account_balance() - self.max_amount_allowed;
        if amount > 0 {
            //Pcreator.transfer(amount);
        }

        //Promise::new(env::current_account_id()).into();
    }
    
    // pub fn creatorKill() {
    //     assert!(msg.sender == creator);
    //     selfdestruct(creator);
    // }


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
        let mut contract = Roulette::new();
        let bets: Vec<Bet> = vec![Bet {
            bet_type: 5,
            number: 0,
            chips: U128::from(10000000000000000)
        }];
        
        // println!("{}", contract.get_status());
        // let bets = 
        // //println!("Value after increment: {}", U128::from(contract.get_status().balance));
        // // confirm that we received 1 when calling get_num
        let number = contract.spin_wheel(bets);
        println!("{}", number);
    }
}
