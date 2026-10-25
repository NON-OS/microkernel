// NONOS Operating System (AGPL-3.0-or-later)
//! A 32-bit unsigned axis past 2^31 is past its logical maximum, and reads
//! as the range's top: it was read as a negative number and told at the
//! other end of the stick.

use crate::descriptors::pad::{pad_layout, PadEvent, PadState};

/// A gamepad with one X axis of 32 bits, logical 0 to 255.
const WIDE: [u8; 18] = [
    0x05, 0x01, 0x09, 0x05, 0xA1, 0x01, // Usage Page 1, Game Pad, Application
    0x15, 0x00, 0x26, 0xFF, 0x00, // Logical 0 to 255
    0x75, 0x20, 0x95, 0x01, 0x09, 0x30, // 32 bits, 1 count, X
    0x81, // Input (data, variable) follows
];

#[test]
fn a_wide_unsigned_axis_past_its_range_reads_as_its_top() {
    let mut d = WIDE.to_vec();
    d.extend([0x02, 0xC0]);
    let mut s = PadState::new(pad_layout(&d).unwrap());
    let mut got = vec![];
    s.feed(&[0xFF, 0xFF, 0xFF, 0xFF], |e| got.push(e));
    assert_eq!(got, vec![PadEvent::Axis(0x30, 32767)]);
    got.clear();
    s.feed(&[0x00, 0x00, 0x00, 0x80], |e| got.push(e));
    assert_eq!(got, vec![], "2^31 too is past the top, and nothing moved");
}
