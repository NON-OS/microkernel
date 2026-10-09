/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used)]

use super::{handed, serves, state, takes_submitter_fee, Handed};

#[test]
fn a_queued_hand_off_carries_its_id_and_a_refusal_its_reason() {
    assert_eq!(
        handed(202, r#"{"id":"a1b2-c3","status":"queued"}"#).unwrap(),
        Handed::Queued { id: "a1b2-c3".into() }
    );
    let refused = handed(400, r#"{"reason":"fee recipient is not the relayer\u0007"}"#).unwrap();
    assert!(matches!(refused, Handed::Refused { reason } if !reason.contains('\u{7}')));
    assert!(handed(202, r#"{"id":"../../etc","status":"queued"}"#).is_err(), "no path in an id");
    assert!(handed(500, "").is_err());
}

#[test]
fn a_state_reads_only_the_statuses_the_relayer_has() {
    let tx = format!("0x{}", "ab".repeat(32));
    let settled = state(&format!(r#"{{"status":"settled","tx":"{tx}"}}"#)).unwrap();
    assert_eq!(settled.tx, Some(tx));
    assert!(state(r#"{"status":"stolen"}"#).is_err());
    assert_eq!(
        state(r#"{"status":"queued","tx":"0x12"}"#).unwrap().tx,
        None,
        "a short hash is dropped"
    );
}

#[test]
fn a_lander_is_used_only_when_it_takes_the_submitter_fee() {
    assert!(takes_submitter_fee(r#"{"format":7,"fee_to_submitter_accepted": true}"#));
    assert!(!takes_submitter_fee(r#"{"format":7,"fee_to_submitter_accepted":false}"#));
    assert!(!takes_submitter_fee(r#"{"format":7}"#));
}

#[test]
fn a_scheduled_hand_off_is_read() {
    assert_eq!(state(r#"{"status":"scheduled"}"#).unwrap().status, "scheduled");
}

/// A lander is used for the pool it names, in any case of the hex, and for no other.
#[test]
fn a_lander_is_used_only_for_the_pool_it_names() {
    let info =
        r#"{"pool":"0xaee51e82965ec1ded870f3f4c248ad4addc3e1cb","fee_to_submitter_accepted":true}"#;
    assert!(serves(info, "0xaEe51E82965Ec1DeD870F3f4c248Ad4AdDc3e1cb"));
    assert!(!serves(info, "0xD0dBCe195c082DA39a218C62c01a732CE5b4d541"));
    assert!(!serves(
        r#"{"fee_to_submitter_accepted":true}"#,
        "0xaEe51E82965Ec1DeD870F3f4c248Ad4AdDc3e1cb"
    ));
}
