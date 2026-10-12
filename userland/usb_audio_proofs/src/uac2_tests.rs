// NONOS Operating System (AGPL-3.0-or-later)
//! USB Audio 2.0 playback: the alternate setting, its clock, and the
//! requests that set and read the clock's rate.

use crate::uac::{clock, get_clock, outputs, playback, select_first, set_clock, Stream, RATE_HZ};
use crate::uac2::*;
use nonos_usbnet::Setup;

#[test]
fn a_2_0_dac_plays_on_its_out_endpoint_past_the_feedback() {
    let p = playback(&plain_dac()).expect("a 2.0 playback alternate");
    let want = Stream {
        interface: 1,
        alt: 1,
        ep_address: 0x01,
        sync: 1,
        max_packet: 200,
        b_interval: 4,
        rate_control: false,
        channels: 2,
        version: 2,
        terminal: 1,
    };
    assert_eq!(p, want);
    assert!(outputs(&plain_dac()).headphone, "2.0 terminals read as 1.0's");
}

#[test]
fn what_the_audio_server_does_not_send_is_refused_in_2_0() {
    let control = [clock_source(SOURCE, 0x07), streaming_in(1, SOURCE)];
    let c: Vec<&[u8]> = control.iter().map(|d| d.as_slice()).collect();
    let with = |g: Vec<u8>, f: Vec<u8>| playback(&dac(&c, &[&g, &f]));
    assert!(with(general2(1, 1, 2), format2(2, 16)).is_some());
    assert!(with(general2(1, 1, 1), format2(2, 16)).is_none(), "mono");
    assert!(with(general2(1, 2, 2), format2(2, 16)).is_none(), "not PCM");
    assert!(with(general2(1, 1, 2), format2(3, 24)).is_none(), "24-bit");
    assert!(with(general2(1, 1, 2), format2(4, 16)).is_none(), "4-byte subslots");
}

#[test]
fn the_clock_is_the_terminals_source_or_a_selectors_first_input() {
    let c = clock(&plain_dac(), 1).expect("a clock");
    assert_eq!((c.interface, c.id, c.programmable, c.selector), (0, SOURCE, true, None));
    let (sel, src, fixed) =
        (clock_selector(0x20, &[0x11, SOURCE]), clock_source(0x11, 1), clock_source(SOURCE, 7));
    let config = dac(&[&src, &fixed, &sel, &streaming_in(1, 0x20)], &[]);
    let c = clock(&config, 1).expect("through the selector");
    assert_eq!((c.id, c.programmable, c.selector), (0x11, false, Some(0x20)));
    assert!(clock(&config, 2).is_none(), "no such terminal");
    let multiplier = [7, 0x24, 0x0C, 0x30, SOURCE, 0, 0];
    assert!(clock(&dac(&[&multiplier, &streaming_in(1, 0x30)], &[]), 1).is_none());
}

#[test]
fn the_clock_requests_name_the_entity_and_the_control_interface() {
    let c = clock(&plain_dac(), 1).expect("a clock");
    let (setup, hz) = set_clock(&c, RATE_HZ);
    assert_eq!((setup, hz), (Setup::new(0x21, 0x01, 0x0100, 0x1000), [0x80, 0xbb, 0, 0]));
    assert_eq!(get_clock(&c), Setup::new(0xA1, 0x01, 0x0100, 0x1000));
    assert_eq!(select_first(&c, 0x20), (Setup::new(0x21, 0x01, 0x0100, 0x2000), [1]));
}
