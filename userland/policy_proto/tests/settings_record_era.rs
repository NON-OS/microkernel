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

//! The settings record across the wallpaper catalog's eras: a choice made by
//! catalog index in another era is not read as the picture now at that index.

use nonos_policy_proto::settings_record::{decode_values, encode, Value, VALUES_MAX};
use nonos_policy_proto::wallpaper_labels::{CATALOG_ERA, WALLPAPER_LABELS};
use nonos_policy_proto::{decode_field, Field};

/// The era entry `encode` puts first: id 0x7F01, kind u8, length 1, the era.
const ENTRY: [u8; 5] = [0x01, 0x7F, 2, 1, CATALOG_ERA];

fn picked() -> Vec<u8> {
    encode([
        (Field::Volume, Value::U8(9)),
        (Field::Wallpaper, Value::U8(2)),
        (Field::WallpapersKept, Value::U64(0b0110)),
    ])
}

fn fields(raw: &[u8]) -> Vec<u32> {
    decode_values(raw).expect("frames").into_iter().map(|(f, _)| f as u32).collect()
}

#[test]
fn a_record_says_its_era_first_and_keeps_its_wallpaper() {
    let raw = picked();
    assert_eq!((&raw[..4], &raw[4..9]), (&b"NSV1"[..], &ENTRY[..]));
    let v = decode_values(&raw).expect("frames");
    assert!(v.contains(&(Field::Wallpaper, Value::U8(2))));
    assert!(v.contains(&(Field::WallpapersKept, Value::U64(0b0110))));
}

#[test]
fn an_earlier_era_keeps_everything_but_its_wallpaper_choice() {
    // As a 0.9.2 build wrote it: the same entries, no era entry.
    let mut raw = picked();
    raw.drain(4..9);
    assert_eq!(fields(&raw), [Field::Volume as u32]);
    // And as a build of another era would say it.
    for era in [0, 1, CATALOG_ERA + 1] {
        let mut raw = picked();
        raw[8] = era;
        assert_eq!(fields(&raw), [Field::Volume as u32], "era {era}");
    }
}

#[test]
fn the_era_entry_is_no_field_and_fits_the_record() {
    assert!(decode_field(0x7F01).is_none(), "an older build skips it");
    assert!(encode([]).len() <= VALUES_MAX);
    assert_eq!(WALLPAPER_LABELS.len(), 4);
}
