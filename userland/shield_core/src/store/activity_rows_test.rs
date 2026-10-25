//! What rows tell beside the history: change, a settled spend, a row's round trip.

use super::super::{from_quad, is_change, settled, to_quad, LANDED, SENT};
use super::row;

#[test]
fn change_is_told_apart_from_a_payment_received() {
    let sent = row(SENT, |a| {
        a.tag = [1; 32];
        a.cm = [4, 4, 4, 4];
    });
    assert!(is_change(&[sent], &[4, 4, 4, 4]));
    assert!(!is_change(&[sent], &[5, 5, 5, 5]));
    let landed = row(LANDED, |a| a.tag = [1; 32]);
    assert!(!settled(&[sent], &[1; 32]));
    assert!(settled(&[sent, landed], &[1; 32]));
}

#[test]
fn a_hash_survives_the_words_a_row_carries() {
    let mut b = [0u8; 32];
    for (i, x) in b.iter_mut().enumerate() {
        *x = u8::try_from(i).unwrap_or(0).wrapping_mul(7);
    }
    assert_eq!(from_quad(&to_quad(&b)), b);
}

#[test]
fn an_activity_row_reads_back_and_an_unknown_kind_is_refused() {
    let a = row(LANDED, |a| {
        a.asset_id = 2;
        a.value = 77;
        a.tx = [3; 32];
        a.tag = [4; 32];
        a.cm = [5, 6, 7, 8];
    });
    let bytes = crate::store::Row::Activity(a).encode();
    let back = crate::store::row::decode(&bytes);
    assert!(matches!(back, Ok(crate::store::Row::Activity(b)) if b == a));
    let mut odd = bytes.clone();
    if let Some(k) = odd.get_mut(1) {
        *k = 99;
    }
    assert!(crate::store::row::decode(&odd).is_err(), "an activity this build does not know");
    let mut short = bytes;
    short.pop();
    assert!(crate::store::row::decode(&short).is_err(), "a row cut short");
}
