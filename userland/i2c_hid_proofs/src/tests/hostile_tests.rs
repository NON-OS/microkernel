// NONOS Operating System (AGPL-3.0-or-later)
//! A report descriptor made hostile: a Report Size of 2^32 bits, a field
//! wider than a value, a field past any report, items cut short, and a
//! Contact Count past the collection. Each yields fewer fields or no
//! frame, never a wrapped sum or a read out of range.

use super::digitizers::*;
use crate::hid::{decode_mouse, decode_touch, parse_mouse_layout, parse_report_descriptor};
use crate::hid::Multitouch;

/// The pad with `pad` (an Input item's globals and the item) before its
/// fingers, in the touch report.
fn with_padding(pad: &[u8]) -> Vec<u8> {
    let mut d = digitizer(PAD, 2);
    d.splice(8..8, pad.iter().copied());
    d
}

#[test]
fn a_report_size_of_2_32_bits_records_nothing_after_it() {
    // Report Size 0xFFFFFFFF, Report Count 2, a constant Input.
    let d = with_padding(&[0x77, 0xFF, 0xFF, 0xFF, 0xFF, 0x95, 0x02, 0x81, 0x03]);
    let layout = parse_report_descriptor(&d);
    assert!(!layout.x.present() && !layout.tip.present());
    let mut mt = Multitouch::new(&d);
    assert!(mt.slots.n == 0 && !mt.serves(), "no finger collection is believed");
    let r = report(&[at(true, 1, 10, 10), at(true, 2, 20, 20)], 2, false);
    assert_eq!(mt.frame(&r).map_or(0, |f| f.n), 0);
    // Neither decode path is taken (the poll asks first), and neither
    // overflows when called anyway.
    let mouse = parse_mouse_layout(&d);
    assert!(!layout.is_absolute_touch() && !mouse.is_relative_mouse());
    let _ = (decode_touch(&r, &layout), decode_mouse(&r, &mouse));
}

#[test]
fn a_field_wider_than_32_bits_or_past_any_report_is_not_recorded() {
    let wide = {
        let mut d = digitizer(PAD, 1);
        let x = d.windows(4).position(|w| w == [0x75, 0x10, 0x09, 0x30]).unwrap();
        d[x + 1] = 0x21; // X of 33 bits
        d
    };
    assert!(!parse_report_descriptor(&wide).x.present());
    // 524288 bits of constant padding first: past the longest report the
    // walk takes, so the descriptor is refused.
    let far = with_padding(&[0x77, 0x00, 0x00, 0x08, 0x00, 0x95, 0x01, 0x81, 0x03]);
    let layout = parse_report_descriptor(&far);
    assert!(!layout.x.present() && !layout.contact_count.present());
}

#[test]
fn items_cut_short_refuse_the_descriptor_whole() {
    let d = digitizer(PAD, 2);
    assert!(parse_report_descriptor(&d).x.present() && Multitouch::new(&d).serves(), "control");
    // A long item claiming 255 bytes, and a four-byte item with no data.
    for cut in [&[0xFE, 0xFF, 0x00][..], &[0x77]] {
        let d = [&d[..], cut].concat();
        assert!(!parse_report_descriptor(&d).x.present(), "a cut descriptor was taken");
        assert_eq!(Multitouch::new(&d).slots.n, 0, "a cut descriptor's fingers were taken");
    }
}

#[test]
fn a_contact_count_past_the_collection_holds_one_frame_back_only() {
    let mut mt = super::multitouch_tests::mt(SCREEN, 2);
    let two = [at(true, 1, 10, 10), at(true, 2, 20, 20)];
    assert!(mt.frame(&report(&two, 200, false)).is_none(), "waits for fingers never sent");
    assert_eq!(mt.frame(&report(&two, 2, false)).map(|f| f.n), Some(2));
}
