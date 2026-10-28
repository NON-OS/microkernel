// NONOS Operating System (AGPL-3.0-or-later)
//! The firmware download: the normal suit for cut B out of the container,
//! its header (without the dynamic part) as the FWHDR_DL command, then every
//! section byte in order in chunks of at most 2020 bytes, on CH12; five
//! tries before the driver gives up, as `rtw89_fw_download` makes.

use crate::fw::IMAGE;
use crate::model::rig;

fn word(b: &[u8], i: usize) -> usize {
    u32::from_le_bytes(b[4 * i..4 * i + 4].try_into().unwrap()) as usize
}

/// Cut B's normal suit (type 5, the container's first entry), by hand.
fn suit_b() -> &'static [u8] {
    let e = &IMAGE[16..32];
    assert_eq!((IMAGE[0], e[0], e[1]), (0xff, 1, 5), "first entry: cut B, normal CE");
    &IMAGE[word(e, 1)..word(e, 1) + word(e, 2)]
}

#[test]
fn the_cut_b_suit_goes_down_whole_and_in_order() {
    let (card, _chip) = rig::ready();
    let s = card.0.borrow();
    let suit = suit_b();
    let count = (word(suit, 6) >> 8) & 0xff;
    let base = 32 + 16 * count;
    assert_eq!(s.fw_hdr, &suit[..base], "the header without its dynamic part");
    let start = if word(suit, 7) & (1 << 16) != 0 { (word(suit, 3) >> 16) & 0xff } else { base };
    assert!(start + s.fw_got.len() <= suit.len());
    assert!(s.fw_got == suit[start..start + s.fw_got.len()], "the sections, byte for byte");
    assert!(s.fw_got.len() > 300_000);
}

#[test]
fn a_failed_download_is_tried_five_times_then_given_up() {
    let (card, got) = rig::bring(|s| s.faults.fw_status = Some(3));
    assert!(got.is_err());
    assert_eq!(card.0.borrow().fw_tries, 5);
}

#[test]
fn a_good_download_is_tried_once_per_power_up() {
    let (card, _chip) = rig::ready();
    // Once at probe, once at mac_init: rtw89 downloads on each partial init.
    assert_eq!(card.0.borrow().fw_tries, 2);
}

#[test]
fn the_firmware_cpu_is_started_only_for_a_download() {
    let (card, _chip) = rig::ready();
    let s = card.0.borrow();
    assert!(!s.violations.iter().any(|v| v.contains("without the download enable")));
    assert!(!s.violations.iter().any(|v| v.contains("download chunk")));
}
