// NONOS Operating System (AGPL-3.0-or-later)
//! A format tag that is not PCM, and descriptor bytes that lie.

use crate::devices::*;
use crate::uac::playback;

#[test]
fn a_non_pcm_format_tag_is_refused() {
    let (ac, alt) = (iface(0, 0, 1, 0), iface(1, 1, 2, 0));
    let (fmt, ep) = (format_i(2, 16, &[48_000], (0, 0)), iso_ep(0x01, 0x09, 192, 1));
    assert!(playback(&config(&[&ac, &alt, &as_general(2), &fmt, &ep])).is_none());
}

#[test]
fn a_descriptor_running_past_the_end_stops_the_walk() {
    let mut c = qemu_usb_audio();
    // The endpoint (9 bytes) and its class descriptor (7) end the config.
    let ep = c.len() - 16;
    c[ep] = 40;
    assert!(playback(&c).is_none());
}
