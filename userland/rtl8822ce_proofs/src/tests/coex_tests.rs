// NONOS Operating System (AGPL-3.0-or-later)
//! The Wi-Fi-only coexistence the 8822C takes: LTE coex off and both TRX
//! masks open in the indirect window, Bluetooth's grant software-low and
//! Wi-Fi's software-high, Wi-Fi the owner of the path, the debug pins off,
//! path B's RF_MODOPT, the scoreboard; a dead window is reported.

use crate::coex::take_antenna;
use crate::model::Chip;
use crate::phy::rf::set_paths;
use crate::regs::Mmio;

/// The indirect writes: (register, value), from CTRL and WDATA.
fn indirect(c: &Chip) -> Vec<(u32, u32)> {
    let w = c.writes.borrow();
    let ctrl = w.iter().enumerate().filter(|(_, x)| x.0 == 0x1700 && x.2 >> 30 == 0b11);
    ctrl.map(|(i, x)| (x.2 & 0xFF, w[..i].iter().rev().find(|y| y.0 == 0x1704).unwrap().2))
        .collect()
}

#[test]
fn wi_fi_takes_the_antennas() {
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    for (r, v) in [(0x40, 0xFFFF_FFFF), (0x64, 0xFFFF_FFFF), (0x70, 0xFFFF_FFFF)] {
        c.set32(r, v);
    }
    assert!(take_antenna(&c));
    // The model's window reads every register back as 0, so each write is
    // its own field: LTE coex off, the TRX masks open, GNT_BT 01 twice (bits
    // 15:14, 11:10), GNT_WL 11 twice (13:12, 9:8).
    let want = [(0x38, 0), (0xA0, 0xFFFF), (0xA4, 0xFFFF), (0x38, 0x4000), (0x38, 0x0400)];
    assert_eq!(indirect(&c), [&want[..], &[(0x38, 0x3000), (0x38, 0x0300)]].concat());
    assert_eq!(c.read8(0x73) & (1 << 2 | 1 << 3), 1 << 2, "path owner Wi-Fi, grant debug off");
    assert_eq!(c.read8(0x66) & 1 << 4, 0);
    assert_eq!(c.read8(0x65) & 1 << 7, 0);
    assert_eq!(c.read16(0x00AA), 0x8003);
    assert_eq!(c.get32(0x4C00 + (0x01 << 2)) & 0xF_FFFF, 0x4_0000);
    assert_eq!(c.read8(0x04C6) & 0x30, 0x10, "PTA TX on, EDCCA off");
}

#[test]
fn a_dead_window_is_reported() {
    let c = Chip::new();
    c.hw.lte_dead.set(true);
    assert!(!take_antenna(&c));
}
