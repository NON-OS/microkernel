/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Version 5 keeps the default network beside everything version 4 kept; a
 * version 4 record an earlier build kept still reads, as the Nym mixnet, and
 * a network this build does not know is refused by name.
 */
//! The wallpapers kept, in setup record version 7: kept across a save, and a
//! set the store would refuse refused here too, desktop wallpaper outside it
//! included. Records from an earlier catalog era: setup_record_era.rs.

use nonos_policy_proto::route::NYM;
use nonos_policy_proto::setup_record::ANSWERS_LEN;
use nonos_policy_proto::setup_record::{check_record, Answers, Host, Name, Record, Refused, Tier};
use nonos_policy_proto::wallpaper_labels::{WALLPAPER_DEFAULT, WALLPAPER_LABELS};
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
fn the_set_round_trips_in_version_7() {
    let set = (1 << 3) | (1 << 1);
    let r = record(3, set);
    let raw = r.encode();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_LEN, &b"NSA7"[..]));
    assert_eq!(check_record(&raw), Ok(r));
}

#[test]
fn a_set_the_store_would_refuse_is_refused() {
    // None kept, one past the collection, and a desktop wallpaper not kept.
    for (wallpaper, set) in [(0, 0), (0, 1 | (1 << 4)), (2, 1 << 3), (4, 1 << 4)] {
        assert_eq!(check_record(&record(wallpaper, set).encode()), Err(Refused::Wallpapers));
    }
}

#[test]
fn the_set_covers_exactly_the_collection() {
    assert_eq!(WALLPAPER_LABELS.len(), 4);
    assert_eq!(ALL, 0b1111);
    assert!(valid(ALL) && valid(1) && !valid(0) && !valid(1 << 4));
    assert!(kept(ALL, 3) && !kept(ALL, 4) && !kept(1, 64));
    assert!(kept(ALL, WALLPAPER_DEFAULT));
}

#[test]
fn stepping_goes_round_the_kept_ones_only() {
    let set = (1 << 0) | (1 << 2) | (1 << 3);
    assert_eq!((next(set, 0), next(set, 2), next(set, 3)), (2, 3, 0));
    assert_eq!((prev(set, 0), prev(set, 2), prev(set, 3)), (3, 0, 2));
    // From one not kept, the next kept after it.
    assert_eq!((next(set, 1), prev(set, 1)), (2, 0));
    // The only one kept stays put.
    assert_eq!((next(1 << 1, 1), prev(1 << 1, 1)), (1, 1));
}
