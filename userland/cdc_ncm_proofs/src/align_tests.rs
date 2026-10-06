// NONOS Operating System (AGPL-3.0-or-later)
//! Where a sent NDP and datagram start, as Linux cdc_ncm_fix_modulus and
//! cdc_ncm_align_tail place them.

use crate::ncm::align::{align_tail, out_align, OutAlign};
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
fn align_tail_rounds_up_adds_the_remainder_and_stops_at_the_end() {
    assert_eq!(align_tail(12, 4, 0, 4095), 12);
    assert_eq!(align_tail(12, 16, 0, 4095), 16);
    assert_eq!(align_tail(28, 4, 2, 4095), 30);
    assert_eq!(align_tail(29, 4, 2, 4095), 34);
    assert_eq!(align_tail(4090, 8, 4, 4095), 4095);
}
