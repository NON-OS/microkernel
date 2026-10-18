//! ETH from a fresh address (v1.4.60): deposits to the pool only, each with
//! value, their values exactly the review's sum, nothing to a token, and
//! the same gas and fee caps.

use super::super::super::*;
use super::{tx, POOL, TOKEN};
use crate::evm::tx::Eip1559;
use ShieldRefusal::*;

fn eth(value: u128) -> Eip1559 {
    Eip1559 { value, ..tx(POOL, vec![0xd0], SHIELD_DEPOSIT_GAS) }
}

#[test]
fn eth_goes_only_as_pool_deposits_whose_values_sum_to_the_review() {
    let ends = (POOL, None);
    let two = [eth(5), eth(2)];
    assert_eq!(shield_checked(&two, ends, None, 7), Ok(()));
    assert_eq!(shield_checked(&two, ends, None, 6), Err(Value), "one wei more than reviewed");
    assert_eq!(shield_checked(&two, ends, None, 8), Err(Value), "one wei less");
    assert_eq!(shield_checked(&[eth(0)], ends, None, 0), Err(Value), "a deposit with no value");
    assert_eq!(shield_checked(&[tx(TOKEN, vec![], 1)], ends, None, 0), Err(NotPool), "no token");
    let wide = [eth(u128::MAX), eth(1)];
    assert_eq!(shield_checked(&wide, ends, None, 0), Err(Value), "no overflow");
    let costly = Eip1559 { gas: SHIELD_DEPOSIT_GAS + 1, ..eth(1) };
    assert_eq!(shield_checked(&[costly], ends, None, 1), Err(FeesPastCaps));
}

#[test]
fn a_nox_batch_never_carries_ether() {
    let nox = (POOL, Some(TOKEN));
    assert_eq!(shield_checked(&[eth(1)], nox, None, 1), Err(Value));
    assert_eq!(shield_checked(&[eth(1)], nox, None, 0), Err(Value));
}
