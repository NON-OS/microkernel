/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Version 5 keeps the default network beside everything version 4 kept; a
 * version 4 record an earlier build kept still reads, as the Nym mixnet, and
 * a network this build does not know is refused by name.
 */
//! The wallpapers kept, in setup record version 6: kept across a save, every
//! wallpaper kept by a record from before the set existed, and a set the
//! store would refuse refused here too, desktop wallpaper outside it included.

use nonos_policy_proto::route::NYM;
use nonos_policy_proto::setup_record::{check_record, Answers, Host, Name, Record, Refused, Tier};
use nonos_policy_proto::setup_record::{ANSWERS_LEN, ANSWERS_V5_LEN};
use nonos_policy_proto::wallpaper_labels::WALLPAPER_LABELS;
use nonos_policy_proto::wallpapers_kept::{kept, next, prev, valid, ALL};

fn record(wallpaper: u8, wallpapers_kept: u64) -> Record {
    let answers = Answers {
        keyboard_layout: 0,
        timezone: 1,
        wallpaper,
        username: Name::new(b"ek").unwrap(),
        qwen_tier: Tier::EMPTY,
    };
    Record { answers, apps_off: 0, hostname: Host::EMPTY, route: NYM, wallpapers_kept }
}

#[test]
fn the_set_round_trips_in_version_6() {
    let set = (1 << 55) | (1 << 62) | 1;
    let r = record(55, set);
    let raw = r.encode();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_LEN, &b"NSA6"[..]));
    assert_eq!(check_record(&raw), Ok(r));
}

#[test]
fn a_version_5_record_keeps_every_wallpaper() {
    let raw = record(55, 1 << 55).encode_v5();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_V5_LEN, &b"NSA5"[..]));
    assert_eq!(check_record(&raw).map(|r| r.wallpapers_kept), Ok(ALL));
}

#[test]
fn a_set_the_store_would_refuse_is_refused() {
    // None kept, one past the collection, and a desktop wallpaper not kept.
    for (wallpaper, set) in [(0, 0), (0, 1 | (1 << 63)), (5, 1 << 6)] {
        assert_eq!(check_record(&record(wallpaper, set).encode()), Err(Refused::Wallpapers));
    }
}

#[test]
fn the_set_covers_exactly_the_collection() {
    assert_eq!(WALLPAPER_LABELS.len(), 63);
    assert_eq!(ALL, (1 << 63) - 1);
    assert!(valid(ALL) && valid(1) && !valid(0) && !valid(1 << 63));
    assert!(kept(ALL, 62) && !kept(ALL, 63) && !kept(1, 64));
}

#[test]
fn stepping_goes_round_the_kept_ones_only() {
    let set = (1 << 3) | (1 << 40) | (1 << 62);
    assert_eq!((next(set, 3), next(set, 40), next(set, 62)), (40, 62, 3));
    assert_eq!((prev(set, 3), prev(set, 40), prev(set, 62)), (62, 3, 40));
    // From one not kept, the next kept after it.
    assert_eq!((next(set, 10), prev(set, 10)), (40, 3));
    // The only one kept stays put.
    assert_eq!((next(1 << 7, 7), prev(1 << 7, 7)), (7, 7));
}
