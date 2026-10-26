// NONOS Operating System (AGPL-3.0-or-later)
//! Every DMA region the driver hands the card must end at or below 4 GiB:
//! the buffer descriptors carry 32-bit addresses.

use crate::fw::dma::{reachable, DMA32_END};

#[test]
fn a_region_ending_at_4_gib_is_reachable() {
    assert!(reachable(DMA32_END - 0x2000, 0x2000));
    assert!(reachable(0, 0x1000));
}

#[test]
fn a_region_crossing_or_above_4_gib_is_not() {
    assert!(!reachable(DMA32_END - 0x1000, 0x2000), "crosses the line");
    assert!(!reachable(DMA32_END, 0x1000), "starts on it");
    assert!(!reachable(0x1_2345_6000, 0x1000), "wholly above");
}

#[test]
fn an_address_that_wraps_is_not_reachable() {
    assert!(!reachable(u64::MAX - 0xFFF, 0x2000));
}
