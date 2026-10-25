/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Each version is known by its own magic, and the apps byte version 3 adds
 * sits past the Qwen tier without reaching into it.
 */

#[path = "common/answers.rs"]
mod answers;

use answers::{kept, record};
use nonos_policy_proto::setup_record::{check_record, Refused, ANSWERS_LEN, ANSWERS_V2_LEN};

#[test]
fn every_version_must_carry_its_own_magic() {
    let mut v3 = record(1).encode_v3();
    v3[3] = b'2';
    assert_eq!(check_record(&v3), Err(Refused::Magic));
    let mut v2 = kept().encode();
    v2[3] = b'3';
    assert_eq!(check_record(&v2), Err(Refused::Magic));
    assert_eq!(check_record(&[0; ANSWERS_LEN]), Err(Refused::Magic));
}

#[test]
fn the_apps_byte_does_not_spill_into_the_tier() {
    let mut raw = record(0xFF).encode();
    assert_eq!(check_record(&raw).unwrap().answers.qwen_tier.as_bytes(), b"small");
    raw[ANSWERS_V2_LEN - 1] = b'x';
    assert_eq!(check_record(&raw), Err(Refused::Tier), "bytes past the tier's end");
}
