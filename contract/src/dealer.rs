
use crate::*;






#[near_bindgen]
impl Contract {

    /*
    stake near to pool
    */
    #[payable]
    pub fn stake(&mut self) {                        
        let sender_id = env::predecessor_account_id();
        let mut account = self.accounts.get(&sender_id).unwrap_or(new_user());
        let now = env::block_timestamp();
        self.round_status.stake_amount = self.round_status.stake_amount + env::attached_deposit();
        if account.stakes.len() == 0 {
            self.stake_accounts.insert(&sender_id.clone());
        }
        account.stakes.push(Stake {
            amount: env::attached_deposit(),
            time: now,
            profit: 0,
            loss: 0
        });
        self.accounts.insert(&sender_id, &account);
        self.cal_max_amount_allowed();
    }

    /*
    unstake from pool
    */
    pub fn unstake(&mut self, index: usize) {
        let sender_id = env::predecessor_account_id();
        let mut account = self.accounts.get(&sender_id).unwrap();
        let mut stake = account.stakes.get_mut(index).unwrap();
        let now = env::block_timestamp();
        let amount = stake.amount + stake.profit - stake.loss;
        assert!(u64::from(stake.time) < u64::from(env::block_timestamp()), "in lock period");
        assert!(amount > 0, "not enough amount!");
        assert!(amount <= env::account_balance(), "not enough balance!");

        
        self.round_status.stake_amount = self.round_status.stake_amount - amount;
        stake.amount -= amount;
        stake.time = now;
        account.stakes.remove(index);
        
        if account.stakes.len() == 0 {
            self.stake_accounts.remove(&sender_id);
        }
        self.accounts.insert(&sender_id, &account);
        self.cal_max_amount_allowed();
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
        contract.stake();
    }
}
