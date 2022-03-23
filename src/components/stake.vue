<template>
  <div class="index" @click="clear_button()"> 
    <div class="header">
      <img class="logo" src="@/assets/logo.png">
      <div class="nav" @click.stop>
        <div class="nav-item" @click="to_game()" v-if="login">GAME</div>
        <div class="nav-item" @click="sign_out()" v-if="login">SIGNOUT</div> 
        <div class="nav-item" @click="sign_in()" v-if="!login">SIGNIN</div>
      </div>
    </div>
    <div class="index-wrapper">
      <div class="line" style="justify-content: center">
        <div class="balance" style="margin-right: 30px;">YOUR STAKE<p><span>{{my_stake_amount}}</span></p></div>
        <div class="bet">TOTAL STAKE<p><span>{{stake_amount}}</span></p></div>
      </div>
      <div class="list">
        <div class="item" v-for="(item, i) in stakes" :key="i">
            <div class="left">
              <div class="stake" style="margin-right: 30px">
                AMOUNT: {{Number(contract.from_yocto(item.amount).replace(/,/g, "")).toFixed(2)}}
              </div>
              <div class="stake" style="margin-right: 30px">
                PROFIT: {{Number(contract.from_yocto(item.profit).replace(/,/g, "")).toFixed(2)}}
              </div>
            </div>
            <div class="wallet-unit" @click.stop>
              <div class="input-wrapper" v-if="unstake_visible[i]">
                <input v-model="unstake_text"/>
              </div>
              <div class="button" @click="toggle_unstake(i)">UNSTAKE</div>
            </div>
            <!--
            <div style="margin-right: 30px">
              <button @click="harvest(i)">HARVEST</button>
            </div>
            -->
        </div>
      </div>
      <div class="line" style="justify-content: center">
        <div class="wallet-wrapper">
          <div class="wallet-unit" @click.stop>
            <div class="input-wrapper" v-if="stake_visible">
              <input v-model="stake_text"/>
            </div>
            <button @click="toggle_stake()">STAKE</button>
          </div>
          <!-- <div class="wallet-unit" @click.stop>
            <div class="input-wrapper">
              <input v-model="withdraw_text"/>
            </div>
            <button @click="toggle_withdraw()">WITHDRAW</button>
          </div> -->
        </div>
      </div>
      <div class="line">
      </div>
    </div>
  </div>
</template>

<script>
import Contract from '../contract'

export default {
  
  data() {
    return {
      login: false,
      accountId: '',
      contract: {},
      status: {},
      stakes: [],
      bet_amount: 0,
      stake_amount: 0,
      stake_visible: false,
      my_stake_amount: 0,
      stake_text: '',
      unstake_visible: [],
      unstake_text: '',
    }
  },

  async mounted() {
    this.contract = new Contract()
    await this.contract.init()
    
    await this.initLogin()
    this.update()
    this.deal_href()
  },

  methods: {
    async initLogin() {
      let login = localStorage.getItem("login")
      let accountId = await this.contract.get_account()
      this.login = login || accountId
      if (this.login) {
        localStorage.setItem("login", true)
      } else {
        localStorage.removeItem("login")
      }
    },

    deal_href() {
      let hash = this.$route.query.transactionHashes
      if (!hash) {
        let index = location.href.indexOf("?")
        if (index > -1) {
          location.href = location.href.substring(0, index)
        }
        return false
      } 
      localStorage.setItem("tx_hash", hash)
      let index = location.href.indexOf("?")
      if (index > -1) {
        location.href = location.href.substring(0, index)
      }
      return true
    },

    async update() {
      if (this.login) {
        let account_status = await this.contract.get_account_status()
        let contract_status = await this.contract.get_contract_status()
        // this.status = status
        console.log(status)
        this.bet_amount = this.contract.from_yocto(contract_status.bet_amount).replace(/,/g, "")
        this.stake_amount = this.contract.from_yocto(contract_status.stake_amount).replace(/,/g, "")
        this.stakes = account_status.stakes
        let total = 0
        this.unstake_visible = []
        for (let i = 0; i < this.stakes.length; i ++) {
          let stake = this.stakes[i]
          total += Number(this.contract.from_yocto(stake.amount).replace(/,/g, ""))
          this.unstake_visible.push(false)
          console.log(total)
        }
        this.my_stake_amount = total
      }
    },

    to_game() {
      this.$router.push('/')
    },

    sign_in() {
      this.contract.sign_in()
    },

    sign_out() {
      this.contract.sign_out()
      this.login = false
    },

    async stake(amount) {
      if (!this.login) {
        this.sign_in()
        return
      }
      await this.contract.stake(amount)
      this.update()
    },

    toggle_stake() {
      this.stake_visible = !this.stake_visible
      if (!this.stake_visible) {
        this.stake(this.stake_text)
      }
    },

    async unstake(amount, index) {
      if (!this.login) {
        this.sign_in()
        return
      }
      try {
        await this.contract.unstake(amount, index)
        this.unstake_text = "";
        window.alert("Unstake Success")
      } catch {
        window.alert("Transaction Expired")
      }
      
      this.update()
    },

    async toggle_unstake(index) {
      for(let i=0;i<this.unstake_visible.length;i++){
        if(i!=index){
           this.unstake_visible[i] = false;
        }
      }
      this.unstake_visible[index] = !this.unstake_visible[index]
      if (this.unstake_visible[index] && this.unstake_text!=0) {
        await this.unstake(this.unstake_text, index)
      }
      this.$forceUpdate()
    },

    async harvest(index) {
      try {
        await this.contract.harvest(index)
        window.alert("Harvest Success")
      } catch {
        window.alert("Transaction Expired")
      }
      this.update()
    },

    async withdraw(amount) {
      if (!this.login) {
        this.sign_in()
        return
      }
      this.editBalance(this.balance - amount)
      await this.contract.withdraw(amount)
      this.update()
    },

    toggle_withdraw() {
      this.withdraw_visible = !this.withdraw_visible
      if (this.withdraw_visible) {
        this.withdraw_text = this.balance_amount
        this.deposit_visible = false
      } else {
        this.withdraw(Number(this.withdraw_text))
      }
    },

    clear_button() {
      this.deposit_visible = false
      this.withdraw_visible = false
    },

  },
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.index {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  min-height: 100vh;
  height: 100%;
  place-items: center;
  background-color: green;
}

.header {
    position: fixed;
    right: 40px;
    top: 20px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .header .logo{
    width:50px;
  }

  .nav{
    display: flex;
  }
  .nav-item{
    margin-left:20px;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size:14px;
    color:#FFF;
    font-weight:bold;
    cursor:pointer;
  }

.index-wrapper {
  width: 100%;
  height: 100%;
  max-height: 30vh;
  max-width: 1200px;
}

.line {
  width: 100%;
  max-width: 1200px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size:16px;
  font-weight:bold;
}

.list{
  max-height:50vh;
  overflow-y:scroll;
  display: flex;
  flex-direction:column;
  align-items:center;
}
.list .item{
  width:75%;
  height:50px;
  margin-top:20px;
  border-radius: 10px;
  display: flex;
  justify-content: space-between;
  border: 1px solid #fff;
  align-items: center;
  font-size:16px;
  color:#FFF;
  padding:0 20px;
}
.list .item .left{
  display: flex;
}


.info {
  display: flex;
  justify-content: space-between;
  color: white;
  width: 330px;
}

.info>div {
    min-width: 75px;
}

.info .padded {
    opacity: .12;
}

p {
    margin: 0;
    padding: 0;
    font-weight: 500;
    font-size: 20px;
}

.actions {
    grid-area: actions;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    -webkit-justify-content: center;
    justify-content: center;
    place-items: center;
}

.actions button {
    opacity: 1;
    transition: opacity .15s;
}

button {
    font-family: "Oswald",sans-serif;
    border: none;
    outline: 1px solid #fff;
    outline-offset: -6px;
    padding: 10px 21px;
    margin: 0;
    text-decoration: none;
    text-transform: uppercase;
    background: transparent;
    color: #fff;
    font-weight: 500;
    font-size: 24px;
    cursor: pointer;
    text-align: center;
    transition: background .25s ease-in-out,-webkit-transform .15s ease;
    transition: background .25s ease-in-out,transform .15s ease;
    transition: background .25s ease-in-out,transform .15s ease,-webkit-transform .15s ease;
}

button svg {
    fill: #fff;
}

button.spin {
    outline: none;
    padding: 0;
    font-weight: 500;
    width: 60%;
    font-size: 18px;
}

.board-wrap>div>div:hover{
  background-color:orange
}

button:hover {
  background: rgba(0,0,0,0.2)
}

.round {
  border-radius: 50%;
}

.placed-chips {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
}

.placed-chips .chip {
    position: relative;
    width: 5.5vw;
    max-width: 30px;
    height: 5.5vw;
    max-height: 30px;
}

.chip text {
    text-anchor: middle;
    font-size: 19px;
    fill: hsla(0,0%,100%,.9);
    font-weight: 500;
}

.wallet-wrapper {
  display: flex;
  justify-content: space-evenly;
  align-items: center;
  margin-top: 90px;
}

.wallet-unit {
  display: flex;
  justify-content: center;
  align-items: center;
}
.wallet-unit .button{
  border:1px solid #FFF;
  border-radius:5px;
  font-size:12px;
  font-weight:bold;
  width:80px;
  height:30px;
  line-height:30px;
  cursor: pointer;
}

.wallet-unit .button:hover{
  background: rgba(0,0,0,0.2);
}

.input-wrapper {
  width: 200px;
  border: none;
  outline: none;
  padding: 0 10px;
  margin: 0;
  text-decoration: none;
  text-transform: uppercase;
  background: rgba(0,0,0,0.2);
  cursor: pointer;
  height:30px;
  line-height:30px;
  transition: background .25s ease-in-out,transform .15s ease;
  margin-right:20px;
}

.input-wrapper input {
    outline-color: invert;
    outline-style: none;
    outline-width: 0px;
    border: none;
    border-style: none;
    text-shadow: none;
    -webkit-appearance: none;
    outline-color: transparent;
    box-shadow: none;
    background-color: transparent;
    text-align: right;
    width: 100%;
    font-family: "Oswald",sans-serif;
    font-weight: 400;
    height:30px;
    line-height:30px;
    color: #fff;
    font-weight: 500;
    font-size: 14px;
    border-radius:5px;
}

.spin-ani {
  animation: spin 5s infinite normal linear;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  50% {
    transform: rotate(180deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.ball-ani {
  animation: ball 1s infinite normal linear;
}

@keyframes ball {
  0% {
    transform: rotate(0deg);
  }
  50% {
    transform: rotate(180deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

</style>
