/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Version 5 keeps the default network beside everything version 4 kept; a
 * version 4 record an earlier build kept still reads, as the Nym mixnet, and
 * a network this build does not know is refused by name.
 */
//! A setup record from an earlier catalog era (version 6 and before) named its
//! wallpaper and the set kept by that era's indices: it is taken with the
//! default wallpaper and every wallpaper kept, and the rest of it as it was.

use nonos_policy_proto::route::NYM;
use nonos_policy_proto::setup_record::ANSWERS_V5_LEN;
use nonos_policy_proto::setup_record::{check_record, Answers, Host, Name, Record, Tier};
use nonos_policy_proto::wallpaper_labels::WALLPAPER_DEFAULT;
use nonos_policy_proto::wallpapers_kept::ALL;

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
fn a_version_6_record_keeps_its_answers_but_not_its_wallpaper() {
    // As 0.9.2 wrote it: special-variant-9 (55) on the desktop, three kept.
    let mut raw = record(55, (1 << 55) | (1 << 62) | (1 << 3)).encode();
    raw[..4].copy_from_slice(b"NSA6");
    let back = check_record(&raw).expect("taken, not refused");
    assert_eq!((back.answers.wallpaper, back.wallpapers_kept), (WALLPAPER_DEFAULT, ALL));
    assert_eq!(back.answers.username.as_bytes(), b"ek", "the rest is kept");
}

#[test]
fn a_version_5_record_keeps_every_wallpaper() {
    let raw = record(55, 1 << 55).encode_v5();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_V5_LEN, &b"NSA5"[..]));
    let back = check_record(&raw).expect("taken");
    assert_eq!((back.answers.wallpaper, back.wallpapers_kept), (WALLPAPER_DEFAULT, ALL));
}
