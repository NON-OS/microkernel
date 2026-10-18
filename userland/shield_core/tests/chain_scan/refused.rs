//! A blob without its committed leaf, or tampered with, is not money.

use super::{abi_bytes, account, index_topic, leaf_of, paid_to, EV};
use nox_shield_core::discovery::{scan_chain, Log};

#[test]
fn a_blob_without_a_committed_leaf_is_not_money() {
    let me = account();
    let (_p, blob) = paid_to(&me, 500);
    let data = abi_bytes(&blob);
    let t = [EV, index_topic(1)];
    let scan = scan_chain(&me, &[], &[Log { topics: &t, data: &data }], &[], &[], &[]);
    assert!(scan.received.is_empty());
}

#[test]
fn a_tampered_blob_is_not_money() {
    let me = account();
    let (plain, mut blob) = paid_to(&me, 500);
    let at = 1500 % blob.len();
    blob[at] ^= 1;
    let ct = [[EV, leaf_of(&plain), index_topic(1)]];
    let data = abi_bytes(&blob);
    let ot = [EV, index_topic(1)];
    let scan = scan_chain(
        &me,
        &[Log { topics: &ct[0], data: &[] }],
        &[Log { topics: &ot, data: &data }],
        &[],
        &[],
        &[],
    );
    assert!(scan.received.is_empty());
}
