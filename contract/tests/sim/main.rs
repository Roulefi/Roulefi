pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U64};
use near_sdk_sim::{call, view, deploy, init_simulator, ContractAccount, UserAccount};
use rust_counter_tutorial::RouletteContract;
use rust_counter_tutorial::{*};
use near_sdk::json_types::{U128};
use near_sdk_sim::to_yocto;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_BYTES => "../out/main.wasm",
}

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;

fn init() -> (UserAccount, ContractAccount<RouletteContract>, UserAccount, UserAccount) {
    let root = init_simulator(None);

    // Deploy the compiled Wasm bytes
    let roulette: ContractAccount<RouletteContract> = deploy!(
        contract: RouletteContract,
        contract_id: "roulette".to_string(),
        bytes: &COUNTER_BYTES,
        signer_account: root
    );

    let alice = root.create_user(
        "alice".parse().unwrap(),
        to_yocto("100")// initial balance

    );

    let bob = root.create_user(
        "bob".parse().unwrap(),
        to_yocto("100")// initial balance

    );

    (root, roulette, alice, bob)
}

#[test]
fn simulate_bet() {
    let (root, roulette, alice, bob) = init();
    call!(
        root,
        roulette.new()
    ).assert_success();
    // Get number on account that hasn't incremented or decremented
    let alice_bets: Vec<Bet> = vec![Bet {
        bet_type: 5,
        number: 0,
        chips: U128::from(10000000000000000000000)
    }];

    call!(
        alice,
        roulette.deposit(U128::from(100000000000000000000000)),
        deposit = 100000000000000000000000
    );
    let number: u8 = call!(
        alice,
        roulette.spin_wheel(alice_bets)
    ).unwrap_json();
    println!("winning number: {}", number);

    let current_status: Status = view!(
        roulette.get_status(alice.account_id.clone())
    ).unwrap_json();
    println!("alice status {:?}", current_status);


    let bob_bets: Vec<Bet> = vec![Bet {
        bet_type: 5,
        number: 0,
        chips: U128::from(10000000000000000000000)
    }];

    let number: u8 = call!(
        bob,
        roulette.spin_wheel(bob_bets),
        deposit = 10000000000000000000000
    ).unwrap_json();
    println!("winning number: {}", number);

    let current_status: Status = view!(
        roulette.get_status(bob.account_id.clone())
    ).unwrap_json();
    println!("bob status {:?}", current_status);

    // let amount: U128 = call!(
    //     alice,
    //     roulette.cash_out()
    // ).unwrap_json();
    // println!("alice take out amount: {}", u128::from(amount));

    let current_status: Status = view!(
        roulette.get_status(alice.account_id.clone())
    ).unwrap_json();
    println!("alice final status {:?}", current_status);

    // current_status = view!(
    //     roulette.get_status()
    // ).unwrap_json();
    // println!("Number after first increment: {}", u128::from(current_status.balance));
    // //assert_eq!(&current_num, &1, "After incrementing, the number should be one.");
}
