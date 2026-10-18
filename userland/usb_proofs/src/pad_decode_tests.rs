// NONOS Operating System (AGPL-3.0-or-later)
//! A gamepad's reports read against its layout: axes scaled and told once
//! and on change, buttons down and up, the hat's null state centred, and
//! another report or a short one never misread.

use crate::descriptors::pad::{pad_layout, PadEvent, PadState};
use crate::pad_layout_tests::PAD;

fn feed(s: &mut PadState, report: &[u8]) -> Vec<PadEvent> {
    let mut out = Vec::new();
    s.feed(report, |e| out.push(e));
    out
}

#[test]
fn the_first_report_tells_every_axis_and_later_ones_only_changes() {
    let mut s = PadState::new(pad_layout(&PAD).unwrap());
    let first = feed(&mut s, &[1, 0, 0, 0, 128, 255, 0, 0x0F]);
    let axes = [(0x30, -32767), (0x31, 128), (0x32, 32767), (0x35, -32767), (0x39, -1)];
    assert_eq!(first, axes.map(|(u, v)| PadEvent::Axis(u, v)).to_vec());
    assert_eq!(feed(&mut s, &[1, 0, 0, 0, 128, 255, 0, 0x0F]), vec![], "nothing moved");
    assert_eq!(feed(&mut s, &[1, 0, 0, 0, 128, 255, 0, 0x02]), vec![PadEvent::Axis(0x39, 2)]);
}

#[test]
fn buttons_go_down_and_up() {
    let mut s = PadState::new(pad_layout(&PAD).unwrap());
    feed(&mut s, &[1, 0, 0, 0, 0, 0, 0, 0]);
    let down = feed(&mut s, &[1, 0x01, 0x08, 0, 0, 0, 0, 0]);
    assert_eq!(down, vec![PadEvent::Button(1, true), PadEvent::Button(12, true)]);
    assert_eq!(feed(&mut s, &[1, 0x00, 0x08, 0, 0, 0, 0, 0]), vec![PadEvent::Button(1, false)]);
    // The padding bits after button 12 are no button.
    assert_eq!(feed(&mut s, &[1, 0x00, 0xF8, 0, 0, 0, 0, 0]), vec![]);
}

#[test]
fn another_report_or_a_short_one_is_not_misread() {
    let mut s = PadState::new(pad_layout(&PAD).unwrap());
    assert_eq!(feed(&mut s, &[2, 0xFF, 0xFF, 1, 2, 3, 4, 5]), vec![], "report 2 is not ours");
    assert_eq!(feed(&mut s, &[1, 0x01]), vec![PadEvent::Button(1, true)], "axes past the data");
    assert_eq!(feed(&mut s, &[]), vec![]);
}

#[test]
fn a_signed_axis_and_a_four_way_hat_scale_to_the_same_range() {
    #[rustfmt::skip]
    let joy = [
        0x05, 0x01, 0x09, 0x04, 0xA1, 0x01, 0x09, 0x30, 0x15, 0x81, 0x25, 0x7F, 0x75, 0x08, 0x95, 0x01,
        0x81, 0x02, 0x09, 0x39, 0x15, 0x00, 0x25, 0x03, 0x75, 0x08, 0x95, 0x01, 0x81, 0x42, 0xC0,
    ];
    let mut s = PadState::new(pad_layout(&joy).expect("a joystick, no report IDs"));
    let e = feed(&mut s, &[0x81, 3]);
    assert_eq!(e, vec![PadEvent::Axis(0x30, -32767), PadEvent::Axis(0x39, 6)], "-127; left");
    assert_eq!(
        feed(&mut s, &[0x7F, 1]),
        vec![PadEvent::Axis(0x30, 32767), PadEvent::Axis(0x39, 2)]
    );
}
