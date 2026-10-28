// NONOS Operating System (AGPL-3.0-or-later)
//! A grant the broker hands back is taken only when it covers what was
//! asked and, for a chip that addresses 32 bits, ends below 4 GiB.

use crate::setup::dma::granted;
use nonos_libc::DmaMapOut;

fn grant(device_addr: u64, length: u64) -> DmaMapOut {
    DmaMapOut { user_va: 0x1000, device_addr, length, grant_id: 1 }
}

#[test]
fn a_whole_grant_below_four_gib_is_taken() {
    assert!(granted(&grant(0x1000_0000, 0x4000), 0x4000, true));
    assert!(granted(&grant(0xFFFF_C000, 0x4000), 0x4000, true), "ends exactly at 4 GiB");
}

#[test]
fn a_short_or_high_grant_is_refused() {
    assert!(!granted(&grant(0x1000_0000, 0x3000), 0x4000, false), "short");
    assert!(!granted(&grant(0xFFFF_D000, 0x4000), 0x4000, true), "crosses 4 GiB");
    assert!(!granted(&grant(0x1_0000_0000, 0x4000), 0x4000, true), "above 4 GiB");
    assert!(granted(&grant(0x1_0000_0000, 0x4000), 0x4000, false), "a 64-bit chip may");
    assert!(!granted(&grant(u64::MAX - 0xFFF, 0x4000), 0x4000, false), "wraps");
}
