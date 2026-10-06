//! A deposit in the form the live pool takes it.
//!
//! The call is pinned against Foundry's own ABI encoder, the fee against the
//! contract's rounding, the stored note against the watcher that must later
//! recognise it, and the pre-flight against the contract's refusals in order.
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::unwrap_used,
    clippy::arithmetic_side_effects
)]

use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::discovery::confirm_deposit;
use nox_shield_core::keys::Account;
use nox_shield_core::wallet::{
    absorb_calldata, check_deposit, prepare_deposit, split_deposit, PoolState, Refusal,
    NATIVE_ASSET,
};

const NOX: u64 = 1_000_000_000_000_000_000;
const NOX_ASSET: u64 = 1;

fn account() -> Account {
    let phrase = generate_phrase().expect("entropy");
    Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account")
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[test]
fn the_call_is_byte_for_byte_what_foundry_encodes() {
    // cast calldata "absorb(uint64,uint256,bytes32)" 1 1000000000000000000 0x1122...1122
    let owner: [u8; 32] = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
        0x66, 0x77, 0x88, 0x99, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00,
        0x11, 0x22,
    ];
    let expected = "5cb945fa\
        0000000000000000000000000000000000000000000000000000000000000001\
        0000000000000000000000000000000000000000000000000de0b6b3a7640000\
        1122334455667788990011223344556677889900112233445566778899001122";
    assert_eq!(hex(&absorb_calldata(NOX_ASSET, u128::from(NOX), &owner)), expected);
}

#[test]
fn the_fee_is_the_contracts_rounding() {
    // fee = amount * bps / 10^4, rounded down, and the note holds the rest.
    assert_eq!(split_deposit(NOX, 25), Some((2_500_000_000_000_000, 997_500_000_000_000_000)));
    assert_eq!(
        split_deposit(399, 25),
        Some((0, 399)),
        "a fee below one base unit rounds to nothing"
    );
    assert_eq!(
        split_deposit(u64::MAX, 25).map(|(f, v)| f + v),
        Some(u64::MAX),
        "no overflow at the ceiling"
    );
    assert_eq!(
        split_deposit(1_000, 10_000),
        None,
        "a fee of everything is refused, as the pool does"
    );
}

#[test]
fn the_kept_note_is_the_leaf_the_pool_will_store() {
    let me = account().address();
    let req = prepare_deposit(&me, u128::from(NOX), NOX_ASSET, 25, 1).expect("deposit");
    assert_eq!(req.note.value, 997_500_000_000_000_000, "the note holds the net value");
    assert_eq!(req.value_wei, 0, "an ERC-20 deposit sends no value");
    assert_eq!(
        &req.calldata[36..68],
        &absorb_calldata(0, u128::from(NOX), &[0; 32])[36..68],
        "the gross amount"
    );
    assert_eq!(&req.calldata[68..], &req.owner_commit);
    // The watcher accepts it at the net value, beside the leaf the call creates.
    confirm_deposit(&req.note, 7, &req.commitment, req.note.value).expect("recognised");
    let native = prepare_deposit(&me, u128::from(NOX), NATIVE_ASSET, 25, 1).expect("native");
    assert_eq!(native.value_wei, u128::from(NOX), "a native deposit sends the amount as value");
}

fn open_pool() -> PoolState {
    PoolState {
        beta_mode: true,
        is_depositor: true,
        addr_cap: 10 * u128::from(NOX),
        total_cap: 100 * u128::from(NOX),
        fee_bps: 25,
        scale: 1,
        ..PoolState::default()
    }
}

#[test]
fn the_pre_flight_refuses_what_the_contract_refuses_in_its_order() {
    assert_eq!(check_deposit(&open_pool(), u128::from(NOX)), Ok(()));
    let with = |f: fn(&mut PoolState)| {
        let mut s = open_pool();
        f(&mut s);
        check_deposit(&s, u128::from(NOX))
    };
    assert_eq!(with(|s| s.deposits_paused = true), Err(Refusal::DepositsPaused));
    assert_eq!(with(|s| s.wound_down = true), Err(Refusal::WoundDown));
    assert_eq!(with(|s| s.beta_paused = true), Err(Refusal::BetaPaused));
    assert_eq!(with(|s| s.is_depositor = false), Err(Refusal::NotOnBetaList));
    assert_eq!(with(|s| s.addr_deposited = 10 * u128::from(NOX)), Err(Refusal::AddressCapReached));
    assert_eq!(with(|s| s.total_deposited = 100 * u128::from(NOX)), Err(Refusal::PoolCapReached));
    assert_eq!(check_deposit(&open_pool(), 0), Err(Refusal::Amount));
    // Paused and wound down together: the contract checks paused first.
    assert_eq!(
        with(|s| {
            s.deposits_paused = true;
            s.wound_down = true;
        }),
        Err(Refusal::DepositsPaused)
    );
}

#[test]
fn the_trap_pool_is_refused_although_it_still_takes_deposits() {
    // 0x4F112D...5c53 is wound down with deposits open. The contract would accept the
    // deposit, and the money could then leave only by refund.
    let trap = PoolState { wound_down: true, ..open_pool() };
    assert_eq!(check_deposit(&trap, u128::from(NOX)), Err(Refusal::WoundDown));
}

#[test]
fn caps_left_at_zero_refuse_everything() {
    // The live format 5 pool today: beta on, NOX caps never set.
    let fresh = PoolState { addr_cap: 0, total_cap: 0, ..open_pool() };
    assert_eq!(check_deposit(&fresh, 1), Err(Refusal::AddressCapReached));
}

/// A launch pool counts NOX in units of `scale` base units. The figures are
/// the contract's: `units = amount / scale`, the fee split in units, the note
/// holding the rest, and the caps counting what was sent.
const SCALE: u128 = 1_000_000_000_000;

fn launch_pool() -> PoolState {
    PoolState { scale: SCALE, open_deposits: true, is_depositor: false, ..open_pool() }
}

#[test]
fn a_launch_deposit_is_split_in_units() {
    let me = account().address();
    let one_and_a_half = 1_500_000_000_000_000_000u128;
    assert_eq!(check_deposit(&launch_pool(), one_and_a_half), Ok(()));
    let req = prepare_deposit(&me, one_and_a_half, NOX_ASSET, 25, SCALE).expect("deposit");
    // 1,500,000 units, and 25 bps of that is 3,750 units of fee.
    assert_eq!(req.note.value, 1_496_250, "the note holds units, not base units");
    assert_eq!(req.fee, 3_750 * SCALE);
    assert_eq!(&req.calldata[36..68], &absorb_calldata(0, one_and_a_half, &[0; 32])[36..68]);
}

#[test]
fn a_launch_deposit_past_a_u64_of_base_units_is_fine() {
    let hundred = 100 * 1_000_000_000_000_000_000u128;
    let pool = PoolState { addr_cap: u128::MAX, total_cap: u128::MAX, ..launch_pool() };
    assert_eq!(check_deposit(&pool, hundred), Ok(()));
}

#[test]
fn a_launch_deposit_must_be_whole_units_and_in_range() {
    assert_eq!(check_deposit(&launch_pool(), SCALE + 1), Err(Refusal::NotWholeUnits));
    assert_eq!(check_deposit(&launch_pool(), SCALE - 1), Err(Refusal::NotWholeUnits));
    let beyond = (u128::from(nox_shield_core::notes::MAX_VALUE) + 1) * SCALE;
    let pool = PoolState { addr_cap: u128::MAX, total_cap: u128::MAX, ..launch_pool() };
    assert_eq!(check_deposit(&pool, beyond), Err(Refusal::Amount));
    assert_eq!(
        check_deposit(&PoolState { scale: 0, ..launch_pool() }, SCALE),
        Err(Refusal::Amount)
    );
}

#[test]
fn open_deposits_lift_the_list_and_nothing_else() {
    assert_eq!(check_deposit(&launch_pool(), SCALE), Ok(()));
    let closed = PoolState { open_deposits: false, ..launch_pool() };
    assert_eq!(check_deposit(&closed, SCALE), Err(Refusal::NotOnBetaList));
    let capped = PoolState { addr_cap: 0, ..launch_pool() };
    assert_eq!(check_deposit(&capped, SCALE), Err(Refusal::AddressCapReached));
}
