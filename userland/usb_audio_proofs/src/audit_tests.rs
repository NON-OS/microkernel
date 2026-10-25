// NONOS Operating System (AGPL-3.0-or-later)
//! A device that lies about its own descriptors: a Format Type I whose
//! sample-rate count its bLength does not hold, and a 2.0 function whose
//! entity ids are reused or 0, so a stream's clock cannot be told.

use crate::devices::{as_general, config, cs_ep, format_i, iface, iso_ep};
use crate::uac::{clock, playback};
use crate::uac2::*;

fn one_alt(format: Vec<u8>) -> Vec<u8> {
    let (ac, alt, ep) = (iface(0, 0, 1, 0), iface(1, 1, 2, 0), iso_ep(0x01, 0x09, 192, 1));
    config(&[&ac, &alt, &as_general(1), &format, &ep, &cs_ep(1)])
}

#[test]
fn a_rate_count_the_format_does_not_hold_plays_nothing() {
    let honest = format_i(2, 16, &[44_100, 48_000], (0, 0));
    assert!(playback(&one_alt(honest.clone())).is_some());
    let mut more = honest.clone();
    more[7] = 3;
    assert!(playback(&one_alt(more)).is_none(), "three rates in room for two");
    // A continuous range needs two rates; one listed rate under count 0.
    let mut range = format_i(2, 16, &[48_000], (0, 0));
    range[7] = 0;
    assert!(playback(&one_alt(range)).is_none(), "a range of one bound");
}

fn dac_of(control: &[Vec<u8>]) -> Vec<u8> {
    let c: Vec<&[u8]> = control.iter().map(|d| d.as_slice()).collect();
    let stream = [general2(1, 1, 2), format2(2, 16)];
    let s: Vec<&[u8]> = stream.iter().map(|d| d.as_slice()).collect();
    dac(&c, &s)
}

#[test]
fn reused_or_zero_entity_ids_name_no_clock() {
    let ok = [clock_source(SOURCE, 7), streaming_in(1, SOURCE), headphones_out(3, 1, SOURCE)];
    assert!(clock(&dac_of(&ok), 1).is_some());
    let twice = [clock_source(SOURCE, 7), streaming_in(1, SOURCE), headphones_out(SOURCE, 1, 0)];
    assert_eq!(clock(&dac_of(&twice), 1), None, "the clock's id reused by a terminal");
    let zero = [clock_source(0, 7), streaming_in(1, 0)];
    assert_eq!(clock(&dac_of(&zero), 1), None, "id 0 names nothing");
}
