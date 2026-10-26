/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Version 3 keeps the apps setup turned off beside the answers, and a
 * version 1 or 2 record an earlier build kept still reads, every app on.
 */

#[path = "common/answers.rs"]
mod answers;

use answers::{kept, record};
use nonos_policy_proto::apps::{BROWSER, LINUX, MEDIA};
use nonos_policy_proto::setup_record::{check, check_record, Answers, Host, Record};
use nonos_policy_proto::setup_record::{ANSWERS_LEN, ANSWERS_V1_LEN, ANSWERS_V2_LEN};
use nonos_policy_proto::wallpaper_labels::WALLPAPER_DEFAULT;
use nonos_policy_proto::wallpapers_kept::ALL;

/* An older version's wallpaper named the first catalog era's: the default. */
fn era_1(a: Answers) -> Answers {
    Answers { wallpaper: WALLPAPER_DEFAULT, ..a }
}

#[test]
fn apps_turned_off_round_trip() {
    let r = record(BROWSER | MEDIA | LINUX);
    let raw = r.encode();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_LEN, &b"NSA7"[..]));
    assert_eq!(Record::decode(&raw), Some(r));
    assert_eq!(check(&raw), Ok(kept()), "the answers read alone as before");
}

#[test]
fn a_version_2_record_reads_with_every_app_on() {
    let raw = kept().encode();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_V2_LEN, &b"NSA2"[..]));
    let want = Record {
        answers: era_1(kept()),
        apps_off: 0,
        hostname: Host::EMPTY,
        route: 0,
        wallpapers_kept: ALL,
    };
    assert_eq!(check_record(&raw), Ok(want));
}

#[test]
fn a_version_1_record_reads_with_every_app_on() {
    let raw: [u8; ANSWERS_V1_LEN] = *b"NSA1\x02\xfb\x03";
    let r = check_record(&raw).unwrap();
    assert_eq!((r.apps_off, r.answers.username.as_bytes()), (0, &b""[..]));
}
