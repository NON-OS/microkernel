// NONOS Operating System (AGPL-3.0-or-later)
//! The SERIAL reply read field by field, and every way it can be short.

use crate::acpi::parse::parse;
use crate::acpi::wire::{request, HidFacts, REPLY_MAX};
use crate::record::{bytes, Rec, KEY};

#[test]
fn a_whole_record_reads_through_its_strings() {
    let s = parse(&bytes(&Rec::default())).unwrap().unwrap();
    assert_eq!((s.count, s.kind, s.chip_select, s.hz, s.pin), (1, 2, 0, 17_000_000, 7));
    assert_eq!((s.gpio_uid, s.key), (Some(2), Some(KEY)));
    assert_eq!((s.data_bits, s.phase, s.polarity, s.mode), (8, 1, 1, 0));
    let hid = HidFacts {
        input_header: 0x1000,
        input_body: 0x1004,
        output: 0x2000,
        read_opcode: 0x0B,
        write_opcode: 0x02,
        flags: 0,
    };
    assert_eq!(s.hid, Some(hid));
    assert!(bytes(&Rec::default()).len() <= REPLY_MAX);
}

#[test]
fn all_ones_is_unknown_and_a_short_hid_block_is_none() {
    let r = Rec { gpio_uid: u64::MAX, key: u64::MAX, present: 0b0_1111, ..Rec::default() };
    let s = parse(&bytes(&r)).unwrap().unwrap();
    assert_eq!((s.gpio_uid, s.key, s.hid), (None, None, None));
}

#[test]
fn a_refusal_is_its_status() {
    assert_eq!(parse(&(-2i32).to_le_bytes()), Some(Err(-2)));
    assert_eq!(parse(&(-1i32).to_le_bytes()), Some(Err(-1)));
    assert_eq!(request(3), [9, 3, 0, 0, 0]);
}

#[test]
fn every_cut_of_a_reply_is_no_record() {
    let b = bytes(&Rec::default());
    for n in 4..b.len() {
        assert_eq!(parse(&b[..n]), None, "cut at {n}");
    }
    assert_eq!(parse(&b[..3]), None);
}

#[test]
fn a_string_running_past_the_end_is_no_record() {
    let mut b = bytes(&Rec::default());
    b[46] = 0xFF;
    assert_eq!(parse(&b), None);
}
