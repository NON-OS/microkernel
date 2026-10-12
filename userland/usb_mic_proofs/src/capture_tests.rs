// NONOS Operating System (AGPL-3.0-or-later)
//! The capture alternate: an IN endpoint and no OUT one, one or two
//! channels, in USB Audio 1.0 and 2.0; and never a playback alternate's
//! clock feedback.

use crate::devices::*;
use crate::uac::find;
use crate::uac2::*;

#[test]
fn a_headsets_mono_microphone_is_found_past_its_playback() {
    let s = find(&headset(), false).expect("a capture alternate");
    assert_eq!((s.interface, s.alt, s.ep_address, s.channels, s.version), (2, 1, 0x81, 1, 1));
    assert_eq!(s.max_packet, 96);
    assert!(find(&qemu_usb_audio(), false).is_none(), "a speaker only");
}

#[test]
fn a_playback_alternate_with_a_sync_endpoint_is_no_microphone() {
    let (ac, alt) = (iface(0, 0, 1, 0), iface(1, 1, 2, 0));
    let (out, sync) = (iso_ep(0x01, 0x05, 192, 1), iso_ep(0x81, 0x01, 3, 1));
    let fmt = format_i(2, 16, &[48_000], (0, 0));
    let c = config(&[&ac, &alt, &as_general(1), &fmt, &out, &cs_ep(1), &sync]);
    assert!(find(&c, true).is_some() && find(&c, false).is_none());
    assert!(find(&plain_dac(), false).is_none(), "2.0 feedback is not capture");
}

fn mic2(channels: u8, attrs: u8) -> Vec<u8> {
    let control = [clock_source(SOURCE, 0x07), headphones_out(2, 1, SOURCE)];
    let c: Vec<&[u8]> = control.iter().map(|d| d.as_slice()).collect();
    let mut parts: Vec<Vec<u8>> = vec![iface(0, 0, 1, 0x20)];
    parts.extend(c.iter().map(|d| d.to_vec()));
    parts.extend([iface(1, 0, 2, 0x20), iface(1, 1, 2, 0x20)]);
    parts.extend([general2(2, 1, channels), format2(2, 16), ep(0x82, attrs, 96, 4), cs_ep2()]);
    let refs: Vec<&[u8]> = parts.iter().map(|p| p.as_slice()).collect();
    config(&refs)
}

#[test]
fn a_2_0_microphone_is_found_with_its_terminal() {
    let s = find(&mic2(1, 0x05), false).expect("a 2.0 capture alternate");
    assert_eq!((s.interface, s.alt, s.ep_address, s.channels, s.version), (1, 1, 0x82, 1, 2));
    assert_eq!(s.terminal, 2, "its clock is looked up through this");
    assert!(find(&mic2(2, 0x05), false).is_some(), "stereo");
    assert!(find(&mic2(3, 0x05), false).is_none(), "three channels");
    assert!(find(&mic2(1, 0x15), false).is_none(), "an explicit feedback endpoint");
}
