/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Version 3 keeps the apps setup turned off beside the answers, and a
 * version 1 or 2 record an earlier build kept still reads, every app on.
 */

use nonos_policy_proto::apps::{BROWSER, LINUX, MEDIA};
use nonos_policy_proto::setup_record::{check, check_record, Answers, Name, Record, Refused, Tier};
use nonos_policy_proto::setup_record::{ANSWERS_LEN, ANSWERS_V1_LEN, ANSWERS_V2_LEN};

fn kept() -> Answers {
    Answers {
        keyboard_layout: 2,
        timezone: -5,
        wallpaper: 3,
        username: Name::new(b"ada").unwrap(),
        qwen_tier: Tier::new(b"small").unwrap(),
    }
}

#[test]
fn apps_turned_off_round_trip() {
    let r = Record { answers: kept(), apps_off: BROWSER | MEDIA | LINUX };
    let raw = r.encode();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_LEN, &b"NSA3"[..]));
    assert_eq!(Record::decode(&raw), Some(r));
    assert_eq!(check(&raw), Ok(kept()), "the answers read alone as before");
}

#[test]
fn a_version_2_record_reads_with_every_app_on() {
    let raw = kept().encode();
    assert_eq!((raw.len(), &raw[..4]), (ANSWERS_V2_LEN, &b"NSA2"[..]));
    assert_eq!(check_record(&raw), Ok(Record { answers: kept(), apps_off: 0 }));
}

#[test]
fn a_version_1_record_reads_with_every_app_on() {
    let raw: [u8; ANSWERS_V1_LEN] = *b"NSA1\x02\xfb\x03";
    let r = check_record(&raw).unwrap();
    assert_eq!((r.apps_off, r.answers.username.as_bytes()), (0, &b""[..]));
}

#[test]
fn every_version_must_carry_its_own_magic() {
    let mut v3 = Record { answers: kept(), apps_off: 1 }.encode();
    v3[3] = b'2';
    assert_eq!(check_record(&v3), Err(Refused::Magic));
    let mut v2 = kept().encode();
    v2[3] = b'3';
    assert_eq!(check_record(&v2), Err(Refused::Magic));
    assert_eq!(check_record(&[0; ANSWERS_LEN]), Err(Refused::Magic));
}

#[test]
fn the_apps_byte_does_not_spill_into_the_tier() {
    let mut raw = Record { answers: kept(), apps_off: 0xFF }.encode();
    assert_eq!(check_record(&raw).unwrap().answers.qwen_tier.as_bytes(), b"small");
    raw[ANSWERS_V2_LEN - 1] = b'x';
    assert_eq!(check_record(&raw), Err(Refused::Tier), "bytes past the tier's end");
}
