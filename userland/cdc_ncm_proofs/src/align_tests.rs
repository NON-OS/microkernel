// NONOS Operating System (AGPL-3.0-or-later)
//! Where a sent NDP and datagram start (NCM 1.0, table 6-3, as recalled,
//! unverified): the device's alignment, divisor and remainder checked, and
//! the first offset that meets them.

use crate::ncm::align::{next, out_align, OutAlign};
use crate::ncm::params::NtbParams;

fn align(divisor: u16, remainder: u16, alignment: u16, tx: usize) -> OutAlign {
    let p = NtbParams {
        out_divisor: divisor,
        out_remainder: remainder,
        out_alignment: alignment,
        ..Default::default()
    };
    out_align(&p, tx)
}

#[test]
fn alignments_are_checked_and_the_remainder_moved_back_by_the_ethernet_header() {
    assert_eq!(align(4, 0, 4, 4095), OutAlign { ndp: 4, modulus: 4, remainder: 2 });
    assert_eq!(align(8, 2, 16, 4095), OutAlign { ndp: 16, modulus: 8, remainder: 4 });
    assert_eq!(
        align(6, 0, 3, 4095),
        OutAlign { ndp: 4, modulus: 4, remainder: 2 },
        "not powers of two"
    );
    assert_eq!(
        align(8192, 0, 8192, 4095),
        OutAlign { ndp: 4, modulus: 4, remainder: 2 },
        "past tx_max"
    );
    assert_eq!(align(4, 5, 4, 4095).remainder, 2, "a remainder past the divisor is 0");
    assert_eq!(align(0, 0, 0, 4095).modulus, 4);
}

#[test]
fn next_is_the_first_offset_with_the_remainder() {
    assert_eq!(next(12, 4, 0), 12);
    assert_eq!(next(12, 16, 0), 16);
    assert_eq!(next(28, 4, 2), 30);
    assert_eq!(next(29, 4, 2), 30, "not a further divisor on");
    assert_eq!(next(30, 4, 2), 30, "already there");
    assert_eq!(next(4090, 8, 4), 4092, "no end clamp: the caller checks the block");
}
