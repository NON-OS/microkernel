//! A launch pool, which counts NOX in units of its scale.

use super::{account, open_pool, NOX_ASSET};
use nox_shield_core::wallet::{
    absorb_calldata, check_deposit, prepare_deposit, PoolState, Refusal,
};

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
