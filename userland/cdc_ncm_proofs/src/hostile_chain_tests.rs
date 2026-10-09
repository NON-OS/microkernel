// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile NDP chains, and arbitrary bytes behind a valid header: the walk
//! ends, takes nothing twice, and reads nothing outside the transfer.

use crate::hostile_tests::good;
use crate::ntb::{found, Ntb, NCM0};

#[test]
fn a_looping_ndp_chain_gives_each_datagram_once() {
    let mut b = good();
    b.u16(18, 12);
    assert_eq!(found(&b.0, 4096), [(40, 60)], "an NDP naming itself");
    let mut b = Ntb::new(256, 12);
    b.ndp(12, NCM0, 200, &[(40, 60)]).ndp(200, NCM0, 12, &[(120, 60)]);
    assert_eq!(found(&b.0, 4096), [(40, 60), (120, 60)], "two NDPs naming each other");
    b.u16(206, 250);
    assert_eq!(found(&b.0, 4096), [(40, 60), (120, 60)], "next NDP past the end");
}

#[test]
fn a_long_ndp_chain_stops_at_linux_loopcount() {
    let mut b = Ntb::new(4096, 12);
    for i in 0..60 {
        let at = 12 + 16 * i;
        b.ndp(at, NCM0, (at + 16) as u16, &[(2000 + i as u16, 14)]);
    }
    assert_eq!(found(&b.0, 4096).len(), 50);
}

#[test]
fn any_bytes_after_a_valid_header_are_read_safely() {
    let mut seed = 0x2545_F491_u32;
    for round in 0..20_000 {
        let len = 20 + (round % 300);
        let mut b = Ntb::new(len, 12);
        for x in b.0[12..].iter_mut() {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            *x = if seed.is_multiple_of(4) { 0 } else { seed as u8 % 64 };
        }
        for (at, n) in found(&b.0, 4096) {
            assert!(at + n <= len && n >= 14);
        }
    }
}
