//! The pre-flight against the contract's refusals, in the contract's order.

use super::{open_pool, NOX};
use nox_shield_core::wallet::{check_deposit, PoolState, Refusal};

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
    assert_eq!(with(|s| s.policy_paused = true), Err(Refusal::PolicyPaused));
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
