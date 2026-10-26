// NONOS Operating System (AGPL-3.0-or-later)
//! Placing the self-relocating image in a 2 MiB block, and the lines a
//! clean by address covers.

use crate::block_layout::{base, span};
use crate::geometry::{dminline_bytes, lines};

const MIB2: u64 = 0x20_0000;
/// The kernel's link address: 0x80000 into its 2 MiB block.
const VIRT_MIN: u64 = 0x4008_0000;

#[test]
fn the_image_keeps_its_offset_in_a_whole_block_inside_the_allocation() {
    let total = 0x579_0000;
    let want = span(total, MIB2).unwrap();
    for at in [0x4000_0000, 0x4000_1000, 0x401F_F000, 0xBB80_0000, 0xBB9F_F000] {
        let b = base(at, VIRT_MIN, MIB2).unwrap();
        assert_eq!(b % MIB2, 0x8_0000, "at {at:#x}");
        assert!(b >= at && b + total <= at + want, "at {at:#x}: {b:#x} spills");
    }
}

#[test]
fn a_bad_alignment_or_an_overflow_is_refused() {
    assert_eq!(span(0x1000, 0x30_0000), None);
    assert_eq!(base(0x1000, VIRT_MIN, 0x30_0000), None);
    assert_eq!(span(u64::MAX - MIB2, MIB2), None);
    assert_eq!(base(u64::MAX - 0x1000, VIRT_MIN, MIB2), None);
}

#[test]
fn dminline_is_log2_words() {
    assert_eq!(dminline_bytes(4 << 16), 64);
    assert_eq!(dminline_bytes(5 << 16), 128);
    assert_eq!(dminline_bytes(0x8444_c004), 64, "a CTR_EL0 a core reports");
}

#[test]
fn every_byte_of_the_range_lies_in_a_counted_line() {
    assert_eq!(lines(0x1000, 0x40, 64), Some((0x1000, 1)));
    assert_eq!(lines(0x103F, 2, 64), Some((0x1000, 2)), "two bytes across a line end");
    assert_eq!(lines(0x1001, 0x40, 64), Some((0x1000, 2)));
    assert_eq!(lines(0x1000, 0, 64), Some((0x1000, 0)));
    assert_eq!(lines(u64::MAX - 0x10, 0x20, 64), None, "a range that wraps");
}
