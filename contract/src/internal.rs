use crate::*;

impl Contract {
    pub(crate) fn cal_share(&self, stake: Stake) -> (U256, U256) {
        let now = env::block_timestamp();
        let time_delta = now - u64::from(stake.time);
        let mut time_rate: u32 = 100;
        for (i, time) in self.config.step_time.iter().enumerate() {
            if time_delta > *time {
                time_rate = 100 + self.config.step_rate[i];
                break;
            }
        }
        let share_numerator = U256::from(stake.amount + stake.profit - stake.loss) * U256::from(time_rate) / U256::from(100 as u128);
        let share_denominator = U256::from(self.round_status.stake_amount + self.round_status.profit_amount - self.round_status.loss_amount);
        (share_numerator, share_denominator)
    }
    /*
    profit needs to calculate every round
    */
    pub(crate) fn cal_profit(&mut self, total_bet: u128, total_win:u128) {
        let mut delta = 0;
        if total_bet >= total_win {
            self.round_status.profit_amount += total_bet - total_win;
            delta = total_bet - total_win;
            let treasury_amount: u128 = (U256::from(delta) * U256::from(self.config.treasury_rate) / U256::from(100 as u128)).as_u128();   // deal with treasury
            self.treasury_status.treasury_amount = self.treasury_status.treasury_amount + treasury_amount;
            delta = delta - treasury_amount;
        } else {
            delta = total_win - total_bet;
            self.round_status.loss_amount += total_win - total_bet;
        }

        if self.round_status.profit_amount >= self.round_status.loss_amount {
            self.round_status.profit_amount -= self.round_status.loss_amount;
            self.round_status.loss_amount = 0;
        } else {
            self.round_status.loss_amount -= self.round_status.profit_amount;
            self.round_status.profit_amount = 0;
        }

        
        for account_id in self.stake_accounts.iter() {                                           // deal with each stake
            let mut account = self.accounts.get(&account_id).unwrap_or(new_user());
            for stake in account.stakes.iter_mut() {
                let (share_numerator, share_denominator) = self.cal_share(stake.clone());
                if total_bet < total_win {     
                    stake.loss = (U256::from(delta) * share_numerator / share_denominator).as_u128();
                } else {
                    stake.profit += (U256::from(delta) * share_numerator / share_denominator).as_u128();
                }

                if stake.profit >= stake.loss {
                    stake.profit -= stake.loss;
                    stake.loss = 0;
                } else {
                    stake.loss -= stake.profit;
                    stake.profit = 0;
                }
                
            }
            self.accounts.insert(&account_id, &account);
        }
        self.round_status.max_amount_allowed = (U256::from(self.round_status.stake_amount + self.round_status.profit_amount) * U256::from(self.config.amount_allowed_rate as u128)).as_u128();   //re-calculate max amount for bets
    
    }
}