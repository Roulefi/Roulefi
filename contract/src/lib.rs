

use std::collections::HashMap;
use std::fmt::Debug;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{BlockHeight, Gas, PanicOnDefault, Promise, env, near_bindgen, BorshStorageKey};
use near_sdk::{AccountId};
use near_sdk::json_types::{U128, U64};
use uint::construct_uint;

near_sdk::setup_alloc!();

pub mod internal;
pub mod roulette;
pub mod dealer;
pub mod view;
pub mod treasury;
use crate::roulette::*;

construct_uint! {
    pub struct U256(4);
}

pub fn new_user() -> Account {
    Account {
        bets: Vec::new(),
        balance: 0,
        stakes: Vec::new(),
        last_bet_time: 0,
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
    owner_id: AccountId,
    config: Config,
    round_status: RoundStatus,
    treasury_status: TreasuryStatus,

    bet_accounts: Vector<AccountId>,  // users who have bets
    stake_accounts: UnorderedSet<AccountId>, // users who have stakes
    accounts: UnorderedMap<AccountId, Account>, // users data
    
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Config {
    treasury_threshold: u128,   // treasury amount threshold
    treasury_shares: Vec<u32>,   // treasury shares
    min_lock_time: u32,         // min time for staking pool to withdraw
    step_time: Vec<u64>,        // a user's share in the pool will rise after each step_time
    step_rate: Vec<u32>,        // the share multiplier for each step_time
    treasury_rate: u32,         // the percentage for every round profit in the pool
    amount_allowed_rate: f32,   // max_amount_allowed = (stake_amount + profit_amount) * amount_allowed_rate
    gas_per_player: u128,
    round_delta: u64
}

/*
round info for spinning
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct RoundStatus {
    current_round_block_index: BlockHeight,
    round_index: BlockHeight,
    next_round_block_index: BlockHeight,
    last_round_win_number: u8,
    spinning: bool,
    max_amount_allowed: u128,
    bet_amount: u128,
    stake_amount: u128,
    profit_amount: u128,
    loss_amount: u128,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct TreasuryStatus {
    last_treasury_time: u64,
    treasury_amount: u128,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct Account {
    bets: Vec<Bet>,               // all bets
    balance: u128,                // user deposit in the contract
    stakes: Vec<Stake>,           // all stakes
    last_bet_time: u64
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    BetAccounts,
    StakeAccounts,
    Accounts
}

const PAYOUTS: [u8; 6] = [2,3,3,2,2,36];
const NUMBER_RANGE: [u8; 6] = [1,2,2,1,1,36];

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        let this = Self {
            owner_id: "bhc.testnet".to_string(),
            config: Config {
                amount_allowed_rate: 0.1,
                min_lock_time: 0,                       // now it is not working
                step_time: vec![0, 604800, 2592000],    // one week , one month
                step_rate: vec![0, 5, 20],        // 5%, 20%
                treasury_rate: 10,
                treasury_threshold: 10000000000000000000000000000,  // 10k
                treasury_shares: vec![40, 40, 20],    // gamers, stake users, team
                gas_per_player: 10000000000000000000000,
                round_delta: 60,
            },
            round_status: RoundStatus {
                round_index: 0,
                current_round_block_index: env::block_index(),
                next_round_block_index: 0,                 //self.round_delta + env::block_index(),
                last_round_win_number: 0,
                max_amount_allowed: 0, ////(env::account_balance() as f64 * 0.1) as u128, 
                bet_amount: 0,
                stake_amount: 0,
                profit_amount: 0,
                loss_amount: 0,
                spinning: false,
            },
            treasury_status: TreasuryStatus {
                last_treasury_time: 0,
                treasury_amount: 0,
            },
            bet_accounts: Vector::new(StorageKey::BetAccounts),
            stake_accounts: UnorderedSet::new(StorageKey::StakeAccounts),
            accounts: UnorderedMap::new(StorageKey::Accounts),

        };
        this
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
            chips: 10000000000000000
        }];
        
        // println!("{}", contract.get_status());
        // let bets = 
        // //println!("Value after increment: {}", U128::from(contract.get_status().balance));
        // // confirm that we received 1 when calling get_num
        let number = 0;//contract.spin_wheel(bets);
        println!("{}", number);
    }
}
