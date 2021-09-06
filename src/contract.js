import "regenerator-runtime/runtime";
import * as nearAPI from "near-api-js";
import getConfig from "./config";
const nearConfig = getConfig("development");
const BET_AMOUNT =  "0000000000000000000000"; /* 0,01 ether, around $6 */
const GAS = "3000000000000"; //300000000000000

export default class Contract {

  near
  wallet_connection
  contract
  status
  provider

  async init() {
    
    this.near = await nearAPI.connect({
        deps: {
        keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore()
        },
        ...nearConfig
    });

    // Needed to access wallet login
    this.wallet_connection = new nearAPI.WalletConnection(this.near);

    // Initializing our contract APIs by contract name and configuration.
    this.contract = await new nearAPI.Contract(this.wallet_connection.account(), nearConfig.contractName, {
        // View methods are read-only – they don't modify the state, but usually return some value
        viewMethods: ['get_status', 'get_round_status'],
        // Change methods can modify the state, but you don't receive the returned value when called
        changeMethods: ['bet', 'spin_wheel', 'deposit', 'withdraw', 'stake', 'unstake', 'harvest'],
        // Sender is the account ID to initialize transactions.
        // getAccountId() will return empty string if user is still unauthorized
        sender: this.wallet_connection.getAccountId()
    });
    this.provider = await new nearAPI.providers.JsonRpcProvider(nearConfig.nodeUrl);
  }

  async is_signed_in() {
    return this.wallet_connection.isSignedIn()
  }

  async sign_in() {
    await this.wallet_connection.requestSignIn(nearConfig.contractName, 'NEAR Roulette');
  }
      
  async sign_out() {
    await this.wallet_connection.signOut()
  }

  async get_account() {
    return await this.wallet_connection.getAccountId()
  }

  async get_status() {
    let accountId = await this.get_account()
    this.status = await this.contract.get_status({sender: accountId})
    return this.status
  }

  async get_round_status() {
    return await this.contract.get_round_status()
  }

  async deposit(amount) {
    let amount_str = amount + BET_AMOUNT
    console.log(amount_str)
    await this.contract.deposit({amount: amount + BET_AMOUNT}, GAS, amount_str)
  }

  async withdraw(amount) {
    if (Number(this.from_yocto(this.status.user.balance)) * 100 < amount) {
      return
    }
    await this.contract.withdraw({amount: amount + BET_AMOUNT});
  }

  async stake(amount) {
    let amount_str = this.to_yocto(amount)
    await this.contract.stake({amount: amount_str}, GAS, amount_str)
  }

  async unstake(amount, index) {
    let amount_str = this.to_yocto(amount)
    await this.contract.unstake({amount: amount_str, index: index})
  }

  async harvest(index) {
    await this.contract.harvest({index: index})
  }

  async bet(bets) {
    let bets_format = []
    let amount = 0
    for (let i = 0; i < bets.length; i++) {
      let bet = {
        bet_type: bets[i].bet_type, 
        number: bets[i].number,
        chips: bets[i].chips + BET_AMOUNT
      }
      amount += (bets[i].chips / 100)

      bets_format.push(bet)
    }
    amount += 0.01
    let balance = Number(this.from_yocto(this.status.user.balance))
    if (balance < amount) {
      let pay = String(parseInt((amount - balance) * 100)) + BET_AMOUNT
      await this.contract.bet({bets: bets_format}, GAS, pay)
    } else {
      await this.contract.bet({bets: bets_format}, GAS)
    }
  }

  async spin_wheel() {
    await this.contract.spin_wheel()
  }

  async get_result(hash) {
    let accountId = await this.get_account()
    let result = await this.provider.sendJsonRpc("EXPERIMENTAL_tx_status", [hash, accountId]);
    let val = nearAPI.providers.getTransactionLastResult(result);
    return val
  }

  

  from_yocto(number) {
    return nearAPI.utils.format.formatNearAmount(number);
  }

  to_yocto(number) {
    return nearAPI.utils.format.parseNearAmount(number)
  }
}