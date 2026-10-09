/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Version 4 keeps the computer's name beside the answers and the apps, and
 * a version 3 record an earlier build kept still reads, with no name.
 */

use nonos_policy_proto::setup_record::{check_record, Answers, Host, Name, Record, Refused, Tier};
use nonos_policy_proto::setup_record::{ANSWERS_LEN, ANSWERS_V3_LEN, ANSWERS_V4_LEN, HOST_MAX};
use nonos_policy_proto::wallpapers_kept::ALL;

fn record(host: &[u8]) -> Record {
    let answers = Answers {
        keyboard_layout: 1,
        timezone: 2,
        wallpaper: 0,
        username: Name::new(b"ada").unwrap(),
        qwen_tier: Tier::new(b"small").unwrap(),
    };
    Record { answers, apps_off: 4, hostname: Host::new(host).unwrap(), route: 0, wallpapers_kept: ALL }
}

#[test]
fn the_computer_name_round_trips() {
    let r = record(b"ada-laptop2");
    let raw = r.encode();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_LEN, &b"NSA6"[..]));
    assert_eq!(Record::decode(&raw), Some(r));
    assert_eq!(check_record(&raw).unwrap().hostname.as_bytes(), b"ada-laptop2");
}

#[test]
fn a_version_3_record_reads_with_no_computer_name() {
    let r = record(b"box");
    let raw = r.encode_v3();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_V3_LEN, &b"NSA3"[..]));
    let back = check_record(&raw).unwrap();
    assert_eq!((back.apps_off, back.hostname), (4, Host::EMPTY));
}

#[test]
fn only_names_the_kernel_takes_can_be_made() {
    assert!(Host::new(b"").is_some());
    assert!(Host::new(&[b'a'; HOST_MAX]).is_some());
    assert!(Host::new(&[b'a'; HOST_MAX + 1]).is_none());
    for bad in [&b"Box"[..], b"1box", b"box-", b"my box", b"box.lan", b"-box"] {
        assert!(Host::new(bad).is_none(), "{:?}", core::str::from_utf8(bad));
    }
}

#[test]
fn a_malformed_computer_name_is_refused_by_name() {
    let mut raw = record(b"box").encode();
    raw[ANSWERS_V3_LEN + 1] = b'B';
    assert_eq!(check_record(&raw), Err(Refused::Host));
    let mut raw = record(b"box").encode();
    raw[ANSWERS_V4_LEN - 1] = b'x';
    assert_eq!(check_record(&raw), Err(Refused::Host), "bytes past the name's end");
    assert_eq!(Refused::Host.name(), "computer name");
}
