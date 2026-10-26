// NONOS Operating System (AGPL-3.0-or-later)
//! The PHY status is read where the card wrote one: after the descriptor and
//! its shift, when the descriptor says so and the driver info holds it.

use crate::rx::desc::parse;

const PHYST: u32 = 1 << 26;

// A descriptor whose first word is `w0`, its PHY status words after it at
// `shift`, and `tail` bytes of frame.
fn slot(w0: u32, shift: usize, tail: usize) -> Vec<u8> {
    let mut b = vec![0u8; 24 + shift];
    b[..4].copy_from_slice(&w0.to_le_bytes());
    for w in [0x1111_1111u32, 0x2222_2222, 0x3333_3333, 0x4444_4444] {
        b.extend_from_slice(&w.to_le_bytes());
    }
    b.extend(vec![0u8; tail]);
    b
}

#[test]
fn the_phy_status_is_read_after_the_descriptor_and_its_shift() {
    let words = Some([0x1111_1111, 0x2222_2222, 0x3333_3333, 0x4444_4444]);
    assert_eq!(parse(&slot(PHYST | 4 << 16, 0, 40)).unwrap().phy, words);
    assert_eq!(parse(&slot(PHYST | 4 << 16 | 2 << 24, 2, 40)).unwrap().phy, words, "shifted");
}

#[test]
fn no_phy_status_unless_flagged_held_and_present() {
    assert_eq!(parse(&slot(4 << 16, 0, 40)).unwrap().phy, None, "not flagged");
    assert_eq!(parse(&slot(PHYST | 1 << 16, 0, 40)).unwrap().phy, None, "8 bytes of driver info");
    assert_eq!(parse(&slot(PHYST | 4 << 16, 0, 0)[..24 + 15]).unwrap().phy, None, "cut short");
}
