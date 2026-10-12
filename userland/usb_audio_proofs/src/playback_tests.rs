// NONOS Operating System (AGPL-3.0-or-later)
//! Finding the playback alternate setting in a configuration.

use crate::devices::*;
use crate::uac::{playback, Stream};

#[test]
fn qemu_usb_audio_plays_on_interface_1_alt_1() {
    let p = playback(&qemu_usb_audio()).expect("a playback alternate");
    let want = Stream {
        interface: 1,
        alt: 1,
        ep_address: 0x01,
        sync: 3,
        max_packet: 192,
        b_interval: 1,
        rate_control: false,
        channels: 2,
        version: 1,
        terminal: 1,
    };
    assert_eq!(p, want);
}

#[test]
fn headset_skips_24_bit_alt_and_the_microphone() {
    let p = playback(&headset()).expect("a playback alternate");
    assert_eq!((p.interface, p.alt, p.ep_address, p.max_packet), (1, 2, 0x02, 192));
    assert_eq!((p.sync, p.rate_control), (2, true));
}

fn one_alt(format: Vec<u8>, protocol: u8, ep: Vec<u8>) -> Vec<u8> {
    let (ac, alt) = (iface(0, 0, 1, 0), iface(1, 1, 2, protocol));
    config(&[&ac, &alt, &as_general(1), &format, &ep, &cs_ep(1)])
}

fn stereo16(rates: &[u32], range: (u32, u32)) -> Vec<u8> {
    format_i(2, 16, rates, range)
}

#[test]
fn a_continuous_range_holding_48k_plays() {
    let ep = iso_ep(0x01, 0x09, 192, 1);
    assert!(playback(&one_alt(stereo16(&[], (8_000, 96_000)), 0, ep.clone())).is_some());
    assert!(playback(&one_alt(stereo16(&[], (8_000, 44_100)), 0, ep)).is_none());
}

#[test]
fn what_the_audio_server_does_not_send_is_refused() {
    let ep = || iso_ep(0x01, 0x09, 192, 1);
    assert!(playback(&one_alt(stereo16(&[44_100], (0, 0)), 0, ep())).is_none());
    assert!(playback(&one_alt(format_i(1, 16, &[48_000], (0, 0)), 0, ep())).is_none());
    assert!(playback(&one_alt(format_i(2, 24, &[48_000], (0, 0)), 0, ep())).is_none());
    // USB Audio 2.0 says protocol 0x20 and lays its descriptors out anew:
    // 1.0's layout under it is not read as 2.0.
    assert!(playback(&one_alt(stereo16(&[48_000], (0, 0)), 0x20, ep())).is_none());
    // An IN endpoint is a capture path; an interrupt one is no stream.
    assert!(
        playback(&one_alt(stereo16(&[48_000], (0, 0)), 0, iso_ep(0x81, 0x09, 192, 1))).is_none()
    );
    assert!(
        playback(&one_alt(stereo16(&[48_000], (0, 0)), 0, iso_ep(0x01, 0x03, 192, 1))).is_none()
    );
    assert!(playback(&one_alt(stereo16(&[48_000], (0, 0)), 0, ep())).is_some());
}
