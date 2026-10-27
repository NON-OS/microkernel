// NONOS Operating System (AGPL-3.0-or-later)
//! The MAC table leaves every register as rtw88's rtw8822c_mac_init and
//! rtw_drv_info_cfg leave it, from any starting contents; its 8822C values
//! are not the RTL8821C's.

use super::mac_ref::rtw88;
use crate::mac::{run_mac_table, MAC_INIT};
use crate::model::Chip;
use crate::regs::Mmio;

/// A chip whose bytes in every register the program touches are `seed`'s.
fn seeded(seed: u64) -> Chip {
    let c = Chip::new();
    let mut x = seed | 1;
    for op in MAC_INIT {
        for b in 0..4 {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            c.mem.borrow_mut().insert(op.offset as usize + b, x as u8);
        }
    }
    c
}

#[test]
fn the_table_leaves_what_rtw88_leaves() {
    for seed in [1, 0xDEAD_BEEF, 0x0123_4567_89AB_CDEF, u64::MAX] {
        let (table, reference) = (seeded(seed), seeded(seed));
        run_mac_table(&table, MAC_INIT);
        rtw88(&reference);
        assert_eq!(*table.mem.borrow(), *reference.mem.borrow(), "seed {seed:#x}");
    }
}

#[test]
fn the_8822c_values_are_its_own() {
    let c = Chip::new();
    run_mac_table(&c, MAC_INIT);
    assert_eq!(c.read32(0x06A0), 0xFFFF_FFFF, "RX filter 0: the 8821C's is 0x0FFFFFFF");
    assert_eq!(c.read32(0x04C8), 0x3F3F_08FF, "aggregation limits 0x3F");
    assert_eq!(c.read8(0x0512), 0x1C, "PIFS");
    assert_eq!(c.read32(0x0304), 0x3333_0000, "interrupt migration");
    assert_eq!(c.read32(0x0608), 0xE400_220E | 1 << 28, "RCR with the PHY status");
    assert_eq!(c.read8(0x060F), 4, "PHY status size");
    assert_eq!(MAC_INIT.len(), 81);
}
