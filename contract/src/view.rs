use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct BetInfo {
    pub bet_type: u8,  
    pub number: u8,
    pub chips: U128,  // 1 chip = 0.01 NEAR
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct StakeInfo {
    pub amount: U128,
    pub time: U64,
    pub profit: U128,
    pub loss: U128,
}

/*
To show the basic user info and contract info
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct AccountStatusInfo {
    bets: Vec<BetInfo>,
    balance: U128,
    stakes: Vec<StakeInfo>                 
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct RoundStatusInfo {
    current_round_block_index: U64,
    round_index: U64,
    next_round_block_index: U64,
    last_round_win_number: u8,
    spinning: bool,       
    bet_amount: U128,           // total bet amount in this round
    bet_count: u32,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct ContractStatusInfo {
    balance: U128,
    max_bet_amount: U128,       // the limit for bet_amount
    stake_amount: U128,         // total stake amount
    profit_amount: U128,    
    loss_amount: U128       
}

#[near_bindgen]
impl Contract {
    pub fn get_account_status(&self, account_id: AccountId) -> AccountStatusInfo {
        let account = self.accounts.get(&account_id).unwrap_or(new_user());
        let mut status = AccountStatusInfo {     // when can we play again
            bets: Vec::new(),
            stakes: Vec::new(),
            balance: U128::from(account.balance)
        };
        for bet in account.bets {
            status.bets.push(BetInfo {
                bet_type: bet.bet_type,  
                number: bet.number,
                chips: U128::from(bet.chips),  // 1 chip = 0.01 NEAR
            })
        }
        for stake in account.stakes {
            status.stakes.push(StakeInfo {
                amount: U128::from(stake.amount),
                time: U64::from(stake.time),
                profit: U128::from(stake.profit),
                loss: U128::from(stake.loss)
            })
        }
        status
    }
    
    pub fn get_contract_status(&self) -> ContractStatusInfo {
        let status = ContractStatusInfo {     
            balance: env::account_balance().into(),
            max_bet_amount: U128::from(self.round_status.max_amount_allowed),
            stake_amount: U128::from(self.round_status.stake_amount),
            profit_amount: U128::from(self.round_status.profit_amount),
            loss_amount: U128::from(self.round_status.loss_amount)
        };
        status
    }

    pub fn get_round_status(&self) -> RoundStatusInfo {
        RoundStatusInfo {
            current_round_block_index: U64::from(self.round_status.current_round_block_index),
            round_index: U64::from(self.round_status.round_index),
            next_round_block_index: U64::from(self.round_status.current_round_block_index + self.config.round_delta),
            bet_amount: U128::from(self.round_status.bet_amount),
            bet_count: self.bet_accounts.len() as u32,
            spinning: self.round_status.spinning,
            last_round_win_number: self.round_status.last_round_win_number,
        }
    }
}