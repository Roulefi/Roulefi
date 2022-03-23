
use std::fmt::Debug;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{PanicOnDefault, Promise, env, log, near_bindgen};
use near_sdk::{AccountId};
use near_sdk::json_types::{U128, U64};
use crate::view::BetInfo;
use crate::*;


/*
check if the bet wins
*/
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
    pub chips: u128,  // 1 chip = 0.01 NEAR
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct Stake {
    pub amount: u128,
    pub time: u64,
    pub profit: u128,
    pub loss: u128,
}



#[near_bindgen]
impl Contract {

    #[payable]
    pub fn bet(&mut self, bets: Vec<BetInfo>, round_index: U64) {
        assert!(!self.round_status.spinning, "wheel spinning, try later");
        assert!(self.round_status.round_index == u64::from(round_index), "uncorrect round index");
        //let prev_storage = env::storage_usage();
        let sender_id = env::predecessor_account_id();
        let mut account = self.accounts.get(&sender_id).unwrap_or(new_user());
        assert!(bets.len() > 0, "you have 0 bets");
        assert!(account.bets.len() == 0, "you've already bet");
        let mut total:u128 = 0;
        for item in bets.iter() {
            total += u128::from(item.chips);
            assert!(item.bet_type <= 5);
            assert!(item.number <= NUMBER_RANGE[item.bet_type as usize]); 
        }
        let balance = account.balance + env::attached_deposit();   // check if user's deposit amount and current trasaction's deposit are greater than the bets
        assert!(balance >= total, "not enough balance"); 
        self.round_status.bet_amount += total;
        assert!(self.round_status.bet_amount < self.round_status.max_amount_allowed, "exceed max bet amount allowed");  // check if total bet amount is greater than the max amount allowed
        account.balance = balance - total;  // the balance decrease when bet is confirmed
        account.bets = bets.iter().map(|bet| {
            Bet {
                chips: u128::from(bet.chips),
                bet_type: bet.bet_type,
                number: bet.number
            }
        }).collect();
        self.accounts.insert(&sender_id, &account);
        self.bet_accounts.push(&sender_id);
    }


    /*
    this method is controlled by a script. the script keeps getting round status until there is any bet
    and time to next_round_block_index, then call this method
    */
    pub fn spin_wheel(&mut self, round_index: U64) {
        assert!(self.round_status.round_index == u64::from(round_index), "uncorrect round index");
        assert!(env::block_index() > self.round_status.current_round_block_index + self.config.round_delta, "too quick to spin");
        assert!(self.bet_accounts.len() > 0, "no bets");
        self.round_status.spinning = true;

        let last_player_id = self.bet_accounts.get(self.bet_accounts.len() - 1).unwrap();
        let bets = self.accounts.get(&last_player_id).unwrap().bets;
        let bet = bets.get(bets.len() - 1).unwrap();
        let nonce: Vec<u8> = vec![bet.number, bet.bet_type];
        let hash = env::sha256(&[env::random_seed(), nonce, last_player_id[..].as_bytes().to_vec()].concat());  //make hash with nonce
        let mut hash_bytes: [u8;4] = [0;4];
        for i in 0..4 {
            hash_bytes[i] = hash.get(i).unwrap().clone();
        }
        let hash_number = u32::from_be_bytes(hash_bytes);   // generate random number
        let number: u8 = hash_number as u8 % 37 ;
        
        let mut total_bet:u128 = 0;
        let mut total_win:u128 = 0;
        for player_id in self.bet_accounts.iter() {                 // check every bet if it wins
            let mut account = self.accounts.get(&player_id).unwrap();
            let mut bet_amount = 0;
            let mut win_amount = 0;
            for b in account.bets.iter() {
                let won = check_win(number, b);
                bet_amount += u128::from(b.chips);
                let mut win_chips = 0;
                if won {
                    win_chips = PAYOUTS[b.bet_type as usize] as u128 * u128::from(b.chips);
                    win_amount += win_chips.clone();
                    
                }      
            }
            account.balance += win_amount;
            total_bet += bet_amount;
            total_win += win_amount;
            account.bets.clear();
            self.accounts.insert(&player_id, &account);
        }
        self.cal_profit(total_bet, total_win);
        
        self.bet_accounts.clear();
        self.round_status.bet_amount = 0;
        self.round_status.current_round_block_index = env::block_index();
        self.round_status.round_index += 1;
        self.round_status.last_round_win_number = number;
        self.round_status.spinning = false;
    }

    /*
    deposit near to play
    */
    #[payable]
    pub fn deposit(&mut self) {                   
        let sender_id = env::predecessor_account_id();
        let mut account = self.accounts.get(&sender_id).unwrap_or(new_user());
        account.balance += env::attached_deposit();
        self.accounts.insert(&sender_id, &account);
    }

    /*
    withdraw from balance
    */
    pub fn withdraw(&mut self, amount: U128) {                  
        let sender_id = env::predecessor_account_id();
        let mut account = self.accounts.get(&sender_id).expect("account not found");
        let amount = u128::from(amount);
        assert!(amount <= account.balance, "not enough balance");
        assert!(amount > 0, "not enough amount!");
        assert!(amount <= env::account_balance(), "not enough balance!");
        account.balance -= amount;
        self.accounts.insert(&sender_id.clone(), &account);
        Promise::new(sender_id).transfer(amount.clone());
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
