/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Version 5 keeps the default network beside everything version 4 kept; a
 * version 4 record an earlier build kept still reads, as the Nym mixnet, and
 * a network this build does not know is refused by name.
 */
//! What Settings changed after setup, kept on a machine that keeps state:
//! every kept field round trips in its own kind, a field this build does not
//! keep is skipped, a value of the wrong kind is never written, and a record
//! that does not frame whole is refused whole.

use nonos_policy_proto::settings_record::{decode_values, encode, Value, KEPT, VALUES_MAX};
use nonos_policy_proto::Field;

/// Values by field id, which prints when a comparison fails.
fn ids<'a>(v: Option<Vec<(Field, Value<'a>)>>) -> Option<Vec<(u32, Value<'a>)>> {
    v.map(|v| v.into_iter().map(|(f, x)| (f as u32, x)).collect())
}

fn every_kept() -> Vec<(Field, Value<'static>)> {
    vec![
        (Field::Username, Value::Str(b"ada")),
        (Field::Hostname, Value::Str(b"ada-laptop")),
        (Field::QwenTier, Value::Str(b"qwen3-8b")),
        (Field::Timezone, Value::I8(-5)),
        (Field::ClockFormat24, Value::Bool(false)),
        (Field::NotificationsEnabled, Value::Bool(true)),
        (Field::WifiRadio, Value::Bool(false)),
        (Field::Wallpaper, Value::U8(62)),
        (Field::WallpapersKept, Value::U64((1 << 62) | (1 << 3))),
        (Field::MouseSensitivity, Value::U8(7)),
        (Field::SoundEnabled, Value::Bool(true)),
        (Field::Volume, Value::U8(40)),
        (Field::AlertSounds, Value::Bool(false)),
        (Field::KernelPreempt, Value::Bool(true)),
        (Field::NetworkRoute, Value::U8(1)),
        (Field::KeyboardLayout, Value::U8(2)),
        (Field::AppsOff, Value::U8(4)),
    ]
}

#[test]
fn every_kept_field_round_trips() {
    let values = every_kept();
    assert_eq!(values.len(), KEPT.len());
    let raw = encode(values.clone());
    assert!(raw.len() <= VALUES_MAX);
    assert_eq!(&raw[..4], b"NSV1");
    assert_eq!(ids(decode_values(&raw)), ids(Some(values)));
}

#[test]
fn a_field_not_kept_or_of_the_wrong_kind_is_not_written() {
    let raw = encode([
        (Field::Persistent, Value::Bool(true)),
        (Field::Volume, Value::Bool(true)),
        (Field::Volume, Value::U8(9)),
    ]);
    assert_eq!(ids(decode_values(&raw)), Some(vec![(Field::Volume as u32, Value::U8(9))]));
}

#[test]
fn an_entry_this_build_does_not_keep_is_skipped() {
    let mut raw = encode([(Field::Volume, Value::U8(9))]);
    // A later build's field, then this one's.
    raw.splice(4..4, [0xEE, 0x7F, 2, 1, 5]);
    assert_eq!(ids(decode_values(&raw)), Some(vec![(Field::Volume as u32, Value::U8(9))]));
}

#[test]
fn a_record_that_does_not_frame_is_refused_whole() {
    let raw = encode([(Field::Volume, Value::U8(9)), (Field::Username, Value::Str(b"ada"))]);
    for cut in 5..raw.len() {
        if cut == 9 {
            continue; // a whole first entry and nothing after it frames
        }
        assert!(decode_values(&raw[..cut]).is_none(), "cut at {cut}");
    }
    assert!(decode_values(b"NSV2").is_none() && decode_values(b"").is_none());
    // A kept field whose length is not its kind's.
    let bad = [b'N', b'S', b'V', b'1', 0x1E, 0x01, 2, 2, 1, 2];
    assert!(decode_values(&bad).is_none());
}
