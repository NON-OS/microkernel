// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile NTB16s: every lie a block can tell about itself is caught
//! before it is acted on, and none of them panics.

use crate::ntb::{found, Ntb, NCM0};

/// A good block: one NDP at 12 with one 60-byte datagram at 40.
pub fn good() -> Ntb {
    let mut b = Ntb::new(128, 12);
    b.ndp(12, NCM0, 0, &[(40, 60)]);
    b
}

#[test]
fn a_block_with_a_bad_header_gives_nothing() {
    assert_eq!(found(&good().0, 4096), [(40, 60)]);
    assert_eq!(found(&good().u32(0, 0x686D_636E).0, 4096), [], "NTH32 not negotiated");
    assert_eq!(found(&good().0, 127), [], "wBlockLength past rx_max");
    assert_eq!(found(&good().0[..19], 4096), [], "shorter than NTH16 and NDP16 header");
    assert_eq!(found(&good().u16(10, 0).0, 4096), [], "no NDP");
    assert_eq!(found(&good().u16(10, 4).0, 4096), [], "NDP inside the NTH16");
    assert_eq!(found(&good().u16(10, 124).0, 4096), [], "NDP header past the end");
    assert_eq!(found(&good().u16(10, 0xFFFF).0, 4096), []);
}

#[test]
fn an_ndp_with_a_bad_length_or_signature_gives_nothing() {
    assert_eq!(found(&good().u16(16, 12).0, 4096), [], "wLength under 16");
    assert_eq!(found(&good().u16(16, 0xFFFC).0, 4096), [], "entries past the end");
    assert_eq!(found(&good().u32(12, 0x1234_5678).0, 4096), [], "not NCM0");
}

#[test]
fn a_bad_datagram_entry_ends_its_ndp_and_keeps_what_came_before() {
    let entry = |index: u16, len: u16| {
        let mut b = Ntb::new(128, 12);
        b.ndp(12, NCM0, 0, &[(40, 20), (index, len)]);
        found(&b.0, 4096)
    };
    assert_eq!(entry(64, 64), [(40, 20), (64, 64)], "ends exactly at the transfer");
    assert_eq!(entry(64, 65), [(40, 20)], "one byte past the end");
    assert_eq!(entry(129, 14), [(40, 20)], "index past the end");
    assert_eq!(entry(0xFFFF, 0xFFFF), [(40, 20)], "index and length wrap");
    assert_eq!(entry(64, 13), [(40, 20)], "shorter than an Ethernet header");
    assert_eq!(entry(64, 0), [(40, 20)], "zero length");
    assert_eq!(found(&good().u16(8, 50).0, 59), [], "longer than rx_max");
}
