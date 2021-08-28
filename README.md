# NEAR-Roulette
> A Near smart contract roulette. 
> 
**NEAR-Roulette** is a regular roulette on Near testnet. You can bet in the game directly or stake your near into the pool then bet passively with gamers.
    
# Roulette
Choose which you want to bet and then submit your bet and transfer some money to the contract, wait for countdown to spin automatically. Each round has 60 blocks to wait, which is approximately 36 seconds. 
If you don't want to transfer money for each bet confirmation, deposit near to the contract. Profit records in contract, it must be withdraw manually if you want.
A maximum bet amount is set every round, to prevent large winning.

# Staking pool
Staking pool is acting like a dealer in the game, players who think playing roulette is risking can stake money in here. When a player wins, the stake users lose their profits or shares, or on the opposite, stake users got profit.
Lock time is provided, stake users can't unstake within lock period. But when staking time last long enough, they got share increased.
Treasury is to separate a few percentage profit to every one in game and pool. 40% for gamers, 40% for stake users, 20% for dev team. once the treasury reach a threshold, the contract tranfers money to the user list.

# Install
## For fronted:
```
yarn
yarn run serve
```
## For contract
```
yarn build:contract
```
## Move wasm file to deploy
```
yarn postbuild
```
## Deploy
```
near deploy
```
## init contract
```
near call xxx new --accountId=xxx
```
