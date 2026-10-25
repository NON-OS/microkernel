// NONOS Operating System (AGPL-3.0-or-later)

//! What Settings kept is read one byte past VALUES_MAX (capsule_policy
//! restore/values.rs) and refused whole when it fills that: a longer record
//! still frames, so cut to VALUES_MAX it would have been applied.

use nonos_policy_proto::kept_values::{kept_values, VALUES_ASK};
use nonos_policy_proto::settings_record::{decode_values, encode, Value, VALUES_MAX};
use nonos_policy_proto::Field;

// A record of exactly `n` bytes: a kept value, then entries of an id this
// build does not know, which framing reads past.
fn record_of(n: usize) -> Vec<u8> {
    let mut raw = encode([(Field::MouseSensitivity, Value::U8(7))]);
    while raw.len() < n {
        let body = (n - raw.len() - 4).min(255);
        raw.extend_from_slice(&[0xFF, 0xFF, 0, body as u8]);
        raw.resize(raw.len() + body, 0);
    }
    raw
}

#[test]
fn a_kept_record_up_to_values_max_is_taken_and_one_byte_more_is_refused() {
    assert_eq!(VALUES_ASK, VALUES_MAX + 1);
    let at_most = record_of(VALUES_MAX);
    assert_eq!(at_most.len(), VALUES_MAX);
    assert_eq!(kept_values(&at_most).map(|v| v.len()), Some(1), "VALUES_MAX taken");
    let over = record_of(VALUES_MAX + 1);
    assert_eq!(over.len(), VALUES_MAX + 1);
    assert!(decode_values(&over).is_some(), "it still frames");
    assert!(kept_values(&over).is_none(), "VALUES_MAX + 1 refused");
}
