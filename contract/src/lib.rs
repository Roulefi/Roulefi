

use std::collections::HashMap;
use std::fmt::Debug;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{BlockHeight, PanicOnDefault, Promise, env, near_bindgen};
use near_sdk::{AccountId};
use near_sdk::json_types::{U128, U64};

near_sdk::setup_alloc!();

pub mod roulette;
pub mod vault;
use crate::roulette::*;
use crate::vault::*;

pub fn new_user() -> User {
    User {
        bets: Vec::new(),
        balance: U128::from(0),
        history_bets: Vec::new(),
        stakes: Vec::new(),
    }
    
}


#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {

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
    pub creator: AccountId,
    pub max_amount_allowed: u128,   // max bet amount per round
    pub amount_allowed_rate: f32,   // max_amount_allowed = (stake_amount + profit_amount) * amount_allowed_rate
    pub number_range: [u8; 6],      // the number range for every type of bet
    pub payouts: [u8; 6],           // 6 types of payout
    pub treasury_threshold: u128,   // treasury amount threshold
    pub treasury_shares: [f32; 3],   // treasury shares
    pub last_treasury_time: u64,
    pub min_lock_time: u32,         // min time for staking pool to withdraw
    pub step_time: Vec<u32>,        // a user's share in the pool will rise after each step_time
    pub step_rate: Vec<f32>,        // the share multiplier for each step_time
    pub treasury_rate: f32,         // the percentage for every round profit in the pool
    pub round_block_index: BlockHeight,  // timestamp when in a new round
    pub round_delta: BlockHeight,   // time period between rounds
    pub round_index: BlockHeight,   // total round index

    pub bet_users: Vec<AccountId>,  // users who have bets
    pub stake_users: Vec<AccountId>, // users who have stakes
    pub win_number: u8,             // win number for the last round
    pub users: HashMap<AccountId, User>, // users data
    pub bet_amount: u128,           // total bet amount in the given round
    pub stake_amount: u128,         // total stake amount
    pub treasury_amount: u128,      // total treasury amount
    pub profit_amount: u128,        // total profit amount
    pub history_numbers: Vec<HistoryNumber>, //history number
    pub history_limit_count: u32,
}

/*
To show the basic user info and contract info
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Status {
    pub balance: U128,              // contract balance
    pub bet_amount: U128,           // total bet amount in this round
    pub max_bet_amount: U128,       // the limit for bet_amount
    pub stake_amount: U128,         // total stake amount
    pub user: User                  // user data
}

/*
round info for spinning
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct RoundStatus {
    pub current_block_index: BlockHeight,
    pub round_index: BlockHeight,
    pub next_round_block_index: BlockHeight,
    pub bet_count: u32,                          // total bet counts
    pub win_number: u8,
    pub history_numbers: Vec<HistoryNumber>
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct User {
    pub bets: Vec<Bet>,               // all bets
    pub balance: U128,                // user deposit in the contract
    pub history_bets: Vec<HistoryRoundBets>,
    pub stakes: Vec<Stake>,           // all stakes
}



#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            creator: "bhc.testnet".to_string(),
            payouts: [2,3,3,2,2,36],
            number_range: [1,2,2,1,1,36],
            amount_allowed_rate: 0.1,
            max_amount_allowed: (env::account_balance() as f64 * 0.1) as u128, 
            min_lock_time: 0,                       // now it is not working
            step_time: vec![0, 604800, 2592000],    // one week , one month
            step_rate: vec![0.0, 0.05, 0.2],        // 5%, 20%
            treasury_rate: 0.1,
            treasury_threshold: 10000000000000000000000000000,  // 10k
            treasury_shares: [0.4, 0.4, 0.2],    // gamers, stake users, team
            last_treasury_time: 0,
            history_limit_count: 20,

            bet_users: Vec::new(),
            stake_users: Vec::new(),
            win_number: 0,
            
            users: HashMap::new(),
            bet_amount: 0,
            stake_amount: 0,
            treasury_amount: 0,
            profit_amount: 0,
            round_block_index: 0,
            round_delta: 60,
            round_index: 0,
            history_numbers: Vec::new()
        }
    }

    // #[init(ignore_state)]
    // pub fn migrate_state(new_data: String) -> Self {
    //     // Deserialize the state using the old contract structure.
    //     let old_contract: OldContract = env::state_read().expect("Old state doesn't exist");
    //     // Verify that the migration can only be done by the owner.
    //     // This is not necessary, if the upgrade is done internally.
    //     assert!(
    //         env::predecessor_account_id() == old_contract.owner_id,
    //         "Can only be called by the owner"
    //     );

    //     // Create the new contract using the data from the old contract.
    //     Self { owner_id: old_contract.owner_id, data: old_contract.data, new_data }
    // }

    pub fn get_status(&self, sender: AccountId) -> Status {
        let user = self.users.get(&sender);
        let user: User = match user {
            Some(v) => v.clone(),
            None => new_user()
        };
        let status = Status {     // when can we play again
            balance: env::account_balance().into(),
            bet_amount: U128::from(self.bet_amount),
            max_bet_amount: U128::from(self.max_amount_allowed),
            stake_amount: U128::from(self.stake_amount),
            
            user: user
        };
        status
    }

    pub fn get_round_status(&self) -> RoundStatus {
        RoundStatus {
            current_block_index: env::block_index(),
            round_index: self.round_index,
            next_round_block_index: (self.round_block_index + self.round_delta) as BlockHeight,
            bet_count: self.bet_users.len() as u32,
            win_number: self.win_number,
            history_numbers: self.history_numbers.clone(),
        }
    }

}

impl Contract { 
    pub(crate) fn deal_history(&mut self) {
        if self.history_numbers.len() > self.history_limit_count as usize {
            let delta = self.history_numbers.len() - self.history_limit_count as usize;
            for i in 0..delta {
                self.history_numbers.remove(i);
            }
        }
        for player_str in self.bet_users.iter() {
            let mut user = self.users.get_mut(player_str).unwrap();
            if user.history_bets.len() > self.history_limit_count as usize {
                let delta = self.history_numbers.len() - self.history_limit_count as usize;
                for i in 0..delta {
                    self.history_numbers.remove(i);
                }
            }
        }
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
