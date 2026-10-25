// NONOS Operating System (AGPL-3.0-or-later)
//! A device's own numbers, refused when no device may give them: an
//! endpoint size or transaction count outside USB's bounds, and UAC2
//! descriptors cut shorter than their fields.

use crate::uac::{clock, playback, timing};
use crate::uac2::*;

const FULL: u8 = 1;
const HIGH: u8 = 3;
const SUPER: u8 = 4;

/// `d` cut to `n` bytes, its bLength saying so.
fn cut(mut d: Vec<u8>, n: usize) -> Vec<u8> {
    d.truncate(n);
    d[0] = n as u8;
    d
}

#[test]
fn reserved_bits_of_the_endpoint_size_are_refused() {
    assert!(timing(HIGH, 4, 192, 2).is_some());
    for bit in 13..16 {
        assert_eq!(timing(HIGH, 4, 1 << bit | 192, 2), None, "bit {bit}");
        assert_eq!(timing(FULL, 1, 1 << bit | 192, 2), None, "bit {bit}");
    }
}

#[test]
fn added_transactions_are_high_speed_only_and_three_is_reserved() {
    assert!(timing(HIGH, 4, 2 << 11 | 100, 2).is_some(), "three of 100 hold 192");
    assert_eq!(timing(HIGH, 4, 3 << 11 | 100, 2), None);
    assert_eq!(timing(SUPER, 4, 1 << 11 | 200, 2), None);
    assert_eq!(timing(FULL, 1, 1 << 11 | 200, 2), None);
}

#[test]
fn a_transaction_past_usbs_isochronous_bound_is_refused() {
    // Four 1 ms frames at full speed: 768 bytes, within either size.
    assert!(timing(FULL, 3, 1023, 2).is_some());
    assert_eq!(timing(FULL, 3, 1024, 2), None);
    assert!(timing(HIGH, 4, 1024, 2).is_some());
    assert_eq!(timing(HIGH, 4, 1025, 2), None);
    assert_eq!(timing(SUPER, 4, 0x7FF, 2), None);
}

fn dac_with(control: &[Vec<u8>], stream: &[Vec<u8>]) -> Vec<u8> {
    let c: Vec<&[u8]> = control.iter().map(|d| d.as_slice()).collect();
    let s: Vec<&[u8]> = stream.iter().map(|d| d.as_slice()).collect();
    dac(&c, &s)
}

#[test]
fn clock_entities_cut_short_name_no_clock() {
    let whole = [clock_source(SOURCE, 7), streaming_in(1, SOURCE), headphones_out(3, 1, SOURCE)];
    let stream = [general2(1, 1, 2), format2(2, 16)];
    assert!(clock(&dac_with(&whole, &stream), 1).is_some());
    for (i, n) in [(0, 7), (1, 16)] {
        let mut c = whole.clone();
        c[i] = cut(c[i].clone(), n);
        assert_eq!(clock(&dac_with(&c, &stream), 1), None, "entity {i} at {n} bytes");
    }
    let no_pins = [clock_selector(SOURCE + 1, &[]), streaming_in(1, SOURCE + 1)];
    assert_eq!(clock(&dac_with(&no_pins, &stream), 1), None, "a selector of no inputs");
}

#[test]
fn streaming_descriptors_cut_short_play_nothing() {
    let control = [clock_source(SOURCE, 7), streaming_in(1, SOURCE)];
    assert!(playback(&dac_with(&control, &[general2(1, 1, 2), format2(2, 16)])).is_some());
    assert_eq!(playback(&dac_with(&control, &[cut(general2(1, 1, 2), 10), format2(2, 16)])), None);
    assert_eq!(playback(&dac_with(&control, &[general2(1, 1, 2), cut(format2(2, 16), 5)])), None);
}
