//! SHIELD_FROM_FRESH (v1.4.56): standard deposits with the remainder left
//! at the address, each refusal by name, and the approval exactly the sum.

use super::super::*;
use crate::evm::tx::Eip1559;
use nonos_nox_config::{APPROVE_GAS, MAX_FEE_PER_GAS, MAX_PRIORITY_FEE_PER_GAS};
use ShieldRefusal::*;

pub(super) const POOL: [u8; 20] = [0x90; 20];
pub(super) const TOKEN: [u8; 20] = [0x70; 20];

pub(super) fn approve(amount: u128) -> Vec<u8> {
    let mut d = vec![0x09, 0x5e, 0xa7, 0xb3];
    d.extend_from_slice(&[0; 12]);
    d.extend_from_slice(&POOL);
    d.extend_from_slice(&[0; 16]);
    d.extend_from_slice(&amount.to_be_bytes());
    d
}

pub(super) fn tx(to: [u8; 20], data: Vec<u8>, gas: u64) -> Eip1559 {
    let (max_fee, max_priority_fee) = (20_000_000_000, 1_000_000_000);
    Eip1559 { chain_id: 11_155_111, nonce: 0, max_priority_fee, max_fee, gas, to, value: 0, data }
}

#[test]
fn only_standard_sizes_go_and_the_rest_stays() {
    let from_five = |s: u64| s >= 5;
    assert_eq!(standard_part(37, |_| true), Some((vec![20, 10, 5, 2], 0)));
    assert_eq!(standard_part(37, from_five), Some((vec![20, 10, 5], 2)), "2 stays there");
    assert_eq!(standard_part(4, from_five), None, "below the smallest");
    assert_eq!(standard_part(0, |_| true), None);
    let (pieces, left) = standard_part(u128::MAX, |s| s == 1).unwrap();
    assert_eq!((pieces.len(), left), (20, u128::MAX - 20), "capped, never overflowing");
}

#[test]
fn the_approval_is_exactly_the_sum_and_deposits_go_to_the_pool() {
    let sum = 37 * 10u128.pow(18);
    let ends = (POOL, Some(TOKEN));
    let ok = tx(TOKEN, approve(sum), APPROVE_GAS);
    assert_eq!(shield_checked(std::slice::from_ref(&ok), ends, Some(&approve(sum)), 0), Ok(()));
    let more = tx(TOKEN, approve(sum + 1), APPROVE_GAS);
    assert_eq!(shield_checked(&[more], ends, Some(&approve(sum)), 0), Err(ApproveNotExact));
    assert_eq!(
        shield_checked(std::slice::from_ref(&ok), ends, None, 0),
        Err(ApproveNotExact),
        "none due"
    );
    let deposit = tx(POOL, vec![1, 2, 3], SHIELD_DEPOSIT_GAS);
    assert_eq!(shield_checked(&[deposit.clone(), deposit.clone()], ends, None, 0), Ok(()));
    assert_eq!(shield_checked(&[tx([0xAB; 20], vec![], 1)], ends, None, 0), Err(NotPool));
    let refused = |t: Eip1559| shield_checked(&[t], ends, None, 0);
    assert_eq!(refused(Eip1559 { value: 1, ..deposit.clone() }), Err(Value));
    assert_eq!(refused(tx(POOL, vec![], SHIELD_DEPOSIT_GAS + 1)), Err(FeesPastCaps));
    assert_eq!(refused(tx(POOL, vec![], 0)), Err(FeesPastCaps));
    let gas =
        shield_checked(&[tx(TOKEN, approve(sum), APPROVE_GAS + 1)], ends, Some(&approve(sum)), 0);
    assert_eq!(gas, Err(FeesPastCaps), "an approval past APPROVE_GAS");
    assert_eq!(refused(Eip1559 { max_fee: 0, ..deposit.clone() }), Err(FeesPastCaps));
    assert_eq!(
        refused(Eip1559 { max_fee: MAX_FEE_PER_GAS + 1, ..deposit.clone() }),
        Err(FeesPastCaps)
    );
    let tip = MAX_PRIORITY_FEE_PER_GAS + 1;
    let high = Eip1559 { max_priority_fee: tip, max_fee: MAX_FEE_PER_GAS, ..deposit.clone() };
    assert_eq!(refused(high), Err(FeesPastCaps));
    assert_eq!(
        refused(Eip1559 { max_priority_fee: deposit.max_fee + 1, ..deposit }),
        Err(FeesPastCaps)
    );
}

#[path = "fresh_test_eth.rs"]
mod eth;
