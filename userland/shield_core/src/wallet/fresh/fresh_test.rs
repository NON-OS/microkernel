/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

//! SIGN_FROM_FRESH's refusals, each by name, and a signature that recovers
//! to the fresh address (v1.4.51).

use super::*;
use FreshRefusal::*;

pub(super) const ESCROW: [u8; 20] = [0xE5; 20];
pub(super) const FRESH: [u8; 20] = [0xF1; 20];
pub(super) const T: &str = "(bytes32,bytes32,bytes32,address,address,uint256,uint64,uint64,bytes32,uint64,bytes32,bytes32,bytes32,bytes32)";

pub(super) fn escrow(chain_id: u64) -> Option<[u8; 20]> {
    (chain_id == 11_155_111).then_some(ESCROW)
}

pub(super) fn refund_data(to: [u8; 20]) -> Vec<u8> {
    let mut d = REFUND.to_vec();
    d.extend_from_slice(&[7u8; 14 * 32]);
    d.extend_from_slice(&[0; 12]);
    d.extend_from_slice(&to);
    d.extend_from_slice(&[9; 32]);
    d
}

pub(super) fn ask(data: Vec<u8>) -> FreshSign {
    FreshSign {
        address: FRESH,
        chain_id: 11_155_111,
        nonce: 0,
        gas: 100_000,
        max_fee: 20_000_000_000,
        max_priority: 1_000_000_000,
        to: ESCROW,
        value: 0,
        data,
    }
}

pub(super) fn refused(r: &FreshSign, handed: Option<u32>) -> FreshRefusal {
    checked(r, handed, escrow).unwrap_err()
}

#[test]
fn each_refusal_by_name() {
    let r = ask(refund_data(FRESH));
    assert_eq!(refused(&r, None), UnknownAddress);
    assert_eq!(refused(&FreshSign { chain_id: 1, ..r.clone() }, Some(4)), NotConfigChain);
    assert_eq!(refused(&FreshSign { to: [0xAA; 20], ..r.clone() }, Some(4)), OtherTo);
    assert_eq!(refused(&FreshSign { value: 1, ..r.clone() }, Some(4)), Value);
    let approve = ask([&[0x09, 0x5e, 0xa7, 0xb3][..], &[0; 64]].concat());
    assert_eq!(refused(&approve, Some(4)), OtherSelector);
    assert_eq!(refused(&ask(Vec::new()), Some(4)), OtherSelector);
    assert_eq!(refused(&ask(refund_data([0xAB; 20])), Some(4)), OtherRefundTo);
    assert_eq!(refused(&ask(refund_data(FRESH)[..470].to_vec()), Some(4)), OtherRefundTo);
}

#[path = "fresh_test_fees.rs"]
mod fees;
#[path = "fresh_test_handed.rs"]
mod handed;
#[path = "fresh_test_once.rs"]
mod once;
#[path = "fresh_test_pass.rs"]
mod pass;
#[path = "fresh_test_rlp.rs"]
mod rlp;
#[path = "fresh_test_sign.rs"]
mod sign;
