use crate::*;



#[near_bindgen]
impl Contract {
    pub fn treasury(&mut self) {
        assert!(self.treasury_status.last_treasury_time < env::block_timestamp(), "too quick for treasury");
        assert!(self.treasury_status.treasury_amount > self.config.treasury_threshold, "not enough treasury");
        let player_amount = U256::from(self.treasury_status.treasury_amount) * U256::from(self.config.treasury_shares[0]) / U256::from(100 as u128);
        let stakers_amount = U256::from(self.treasury_status.treasury_amount) * U256::from(self.config.treasury_shares[1]) / U256::from(100 as u128);
        let team_amount = U256::from(self.treasury_status.treasury_amount) * U256::from(self.config.treasury_shares[2]) / U256::from(100 as u128);
        let mut player_count = 0;
        for (_, account) in self.accounts.iter() {
            if u64::from(account.last_bet_time) > self.treasury_status.last_treasury_time {
                player_count += 1
            }
        }
        let each_player_amount = player_amount.as_u128() / player_count;

        for (account_id, account) in (&mut self.accounts).to_vec().clone().iter_mut()   {
            if account.last_bet_time > self.treasury_status.last_treasury_time {
                account.balance += each_player_amount;
            }
            for stake in account.stakes.iter_mut() {
                let (share_numerator, share_denominator) = self.cal_share(stake.clone());
                let profit = (U256::from(stakers_amount) * share_numerator / share_denominator).as_u128();
                stake.profit += profit;
            }
            self.accounts.insert(&account_id, &account);
        }
        Promise::new(self.owner_id.clone()).transfer(team_amount.as_u128());

        self.treasury_status.treasury_amount = 0;
        self.treasury_status.last_treasury_time = env::block_timestamp();
    }

    
}

