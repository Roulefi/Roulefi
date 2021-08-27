pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U64};
use near_sdk_sim::{call, view, deploy, init_simulator, ContractAccount, UserAccount};
use near_roulette::ContractContract;
use near_roulette::{*, roulette::*, vault::*};
use near_sdk::json_types::{U128};
use near_sdk_sim::to_yocto;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_BYTES => "../out/main.wasm",
}

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;

fn init() -> (UserAccount, ContractAccount<ContractContract>) {
    let root = init_simulator(None);
    let roulette: ContractAccount<ContractContract> = deploy!(
        contract: ContractContract,
        contract_id: "contract",
        bytes: &COUNTER_BYTES,
        signer_account: root
    );

    

    (root, roulette)
}

#[test]
fn simulate_bet() {
    let (root, roulette) = init();

    let alice = root.create_user(
        "alice".parse().unwrap(),
        to_yocto("10000")// initial balance

    );

    let bob = root.create_user(
        "bob".parse().unwrap(),
        to_yocto("10000")// initial balance
    );

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
    ).assert_success();

    call!(
        alice,
        roulette.bet(alice_bets)
    ).assert_success();
    // call!(
    //     root,
    //     roulette.spin_wheel()
    // ).assert_success();

    // let current_status: Status = view!(
    //     roulette.get_status(alice.account_id.clone())
    // ).unwrap_json();
    // println!("alice status {:?}", current_status);


    // let bob_bets: Vec<Bet> = vec![Bet {
    //     bet_type: 5,
    //     number: 0,
    //     chips: U128::from(10000000000000000000000)
    // }];

    // call!(
    //     root,
    //     roulette.spin_wheel()
    // ).assert_success();

    // let current_status: Status = view!(
    //     roulette.get_status(bob.account_id.clone())
    // ).unwrap_json();
    // println!("bob status {:?}", current_status);

    // // let amount: U128 = call!(
    // //     alice,
    // //     roulette.cash_out()
    // // ).unwrap_json();
    // // println!("alice take out amount: {}", u128::from(amount));

    // let current_status: Status = view!(
    //     roulette.get_status(alice.account_id.clone())
    // ).unwrap_json();
    // println!("alice final status {:?}", current_status);

    // current_status = view!(
    //     roulette.get_status()
    // ).unwrap_json();
    // println!("Number after first increment: {}", u128::from(current_status.balance));
    // //assert_eq!(&current_num, &1, "After incrementing, the number should be one.");
}

#[test]
fn simulate_stake() {
    let (root, roulette) = init();

    let alice = root.create_user(
        "alice".parse().unwrap(),
        to_yocto("10000")// initial balance

    );

    let bob = root.create_user(
        "bob".parse().unwrap(),
        to_yocto("10000")// initial balance
    );

    let jimmy = root.create_user(
        "jimmy".parse().unwrap(),
        to_yocto("10000")// initial balance
    );

    let douchebag = root.create_user(
        "douchebag".parse().unwrap(),
        to_yocto("10000")// initial balance
    );

    call!(
        root,
        roulette.new()
    ).assert_success();

    call!(
        alice,
        roulette.stake(U128::from(to_yocto("100"))),
        deposit = to_yocto("100")
    ).assert_success();

    call!(
        jimmy,
        roulette.stake(U128::from(to_yocto("100"))),
        deposit = to_yocto("100")
    ).assert_success();

    call!(
        douchebag,
        roulette.stake(U128::from(to_yocto("100"))),
        deposit = to_yocto("100")
    ).assert_success();

    let bob_bets: Vec<Bet> = vec![Bet {
        bet_type: 5,
        number: 1,
        chips: U128::from(to_yocto("1"))
    }];

    call!(
        bob,
        roulette.bet(bob_bets),
        deposit = to_yocto("1")
    ).assert_success();

    call!(
        root,
        roulette.spin_wheel()
    ).assert_success();

    let current_status: Status = view!(
        roulette.get_status(alice.account_id.clone())
    ).unwrap_json();
    println!("alice status {:?}", current_status);

    let current_status: Status = view!(
        roulette.get_status(bob.account_id.clone())
    ).unwrap_json();
    println!("bob status {:?}", current_status);

    let round_status: RoundStatus = view! (
        roulette.get_round_status()
    ).unwrap_json();
    println!("round status {:?}", round_status);
}
