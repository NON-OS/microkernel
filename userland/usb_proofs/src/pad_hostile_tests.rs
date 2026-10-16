// NONOS Operating System (AGPL-3.0-or-later)
//! A gamepad descriptor bent by a hostile device, beyond the counts and
//! pushes pad_layout_tests.rs bounds: a reserved or too-wide report ID, a
//! usage range inverted or run across two pages, more usages than fields,
//! and a Report Count whose bits overflow, which saturates and never wraps
//! a later field back over an earlier one.

use crate::descriptors::pad::pad_layout;
use crate::pad_layout_tests::PAD;

#[test]
fn a_reserved_or_wide_report_id_names_no_report() {
    let mut zero = PAD;
    zero[7] = 0x00;
    assert_eq!(pad_layout(&zero), None, "ID 0");
    let wide = [&PAD[..6], &[0x86, 0x00, 0x01], &PAD[8..]].concat();
    assert_eq!(pad_layout(&wide), None, "ID 256 is no byte");
}

#[test]
fn an_inverted_or_cross_page_range_gives_no_buttons() {
    let mut inverted = PAD;
    (inverted[11], inverted[13]) = (0x0C, 0x01);
    let l = pad_layout(&inverted).expect("its axes still");
    assert!(l.fields.iter().all(|f| f.page != 0x09), "no buttons from 12 down to 1");
    // Usage Minimum Button 1, Usage Maximum Generic Desktop 12, each in its
    // four-byte form with its own page.
    let range = [0x1B, 0x01, 0x00, 0x09, 0x00, 0x2B, 0x0C, 0x00, 0x01, 0x00];
    let across = [&PAD[..10], &range[..], &PAD[14..]].concat();
    let l = pad_layout(&across).expect("its axes still");
    assert_eq!(l.fields.len(), 4 + 1, "no field from a range across two pages");
}

#[test]
fn more_usages_than_fields_is_no_gamepad() {
    let many: Vec<u8> = (0..65).flat_map(|_| [0x09, 0x30]).collect();
    let head = [0x05, 0x01, 0x09, 0x05, 0xA1, 0x01, 0x05, 0x01];
    let tail = [0x75, 0x08, 0x95, 0x01, 0x81, 0x02, 0xC0];
    assert_eq!(pad_layout(&[&head[..], &many, &tail].concat()), None);
    let fits = [&head[..], &many[..128], &tail].concat();
    assert!(pad_layout(&fits).is_some(), "64 are taken");
}

#[test]
fn bits_past_u32_saturate_and_never_wrap_onto_earlier_fields() {
    // The padding's Report Size 32 and Count 0x7FFFFFFF: its bits pass u32.
    let mut huge = PAD.to_vec();
    huge.splice(24..28, [0x75, 0x20, 0x97, 0xFF, 0xFF, 0xFF, 0x7F]);
    let l = pad_layout(&huge).expect("still a pad");
    let x = l.fields.iter().find(|f| f.usage == 0x30).expect("X");
    assert_eq!(x.bit_offset, u32::MAX, "saturated, not wrapped to a small offset");
}
