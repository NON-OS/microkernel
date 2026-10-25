//! The fee offered from a block and a tip, and the gas limit.

use super::super::reply::one;
use super::*;

#[test]
fn the_ceiling_is_twice_the_base_fee_plus_the_tip() {
    let block = one(r#"{"id":1,"result":{"baseFeePerGas":"0x3b9aca00","number":"0x1"}}"#).unwrap();
    let tip = one(r#"{"id":2,"result":"0x5f5e100"}"#).unwrap();
    let f = fees(&block, &tip).unwrap().unwrap();
    assert_eq!(f, Fees { max_priority_fee: 100_000_000, max_fee: 2_100_000_000 });
}

#[test]
fn an_absurd_fee_is_refused_and_ether_to_a_person_is_21000() {
    let block = one(r#"{"id":1,"result":{"baseFeePerGas":"0xe8d4a51000"}}"#).unwrap();
    let tip = one(r#"{"id":2,"result":"0x1"}"#).unwrap();
    assert_eq!(fees(&block, &tip).unwrap(), None);
    let calm = one(r#"{"id":1,"result":{"baseFeePerGas":"0x1"}}"#).unwrap();
    let greedy = one(r#"{"id":2,"result":"0x2540be400"}"#).unwrap();
    assert_eq!(fees(&calm, &greedy).unwrap().unwrap().max_priority_fee, 5_000_000_000);
    assert_eq!(limit(99_999, true), Some(21_000));
    assert_eq!(limit(40_000, false), Some(50_000));
}
