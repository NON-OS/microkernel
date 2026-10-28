// NONOS Operating System (AGPL-3.0-or-later)
//! The PCIe steps rtw88 takes for the 8822C alone: the reference-clock delay
//! off through DBI, `BIT_CF_L_V2` for front end 5 only, and the auxiliary
//! clock bit from cut D on.

use crate::model::Chip;
use crate::pcie::{after_mac, board, clock_delay_off};
use crate::regs::Mmio;

#[test]
fn the_clock_delay_is_written_zero_through_dbi() {
    let c = Chip::new();
    c.mem.borrow_mut().insert(0x03E9, 0xAB);
    assert!(clock_delay_off(&c));
    // 0x725: dword 0x724, byte 1 enabled (bit 13), data in WDATA byte 1.
    assert_eq!(c.at(0x03E9), [0]);
    assert_eq!(c.at(0x03F0), [0x0724 | 1 << 13]);
    assert_eq!(c.at(0x03F2), [1]);
}

#[test]
fn a_dbi_write_that_never_completes_is_said() {
    let c = Chip::new();
    c.hw.dbi_stuck.set(true);
    assert!(!clock_delay_off(&c));
}

#[test]
fn front_end_5_alone_sets_cf_l() {
    for rfe in 0..=6u8 {
        let c = Chip::new();
        c.set32(0x1010, 0xFFFF_FFFF);
        board(&c, rfe);
        let want = if rfe == 5 { 0xDFFF_FFFF } else { 0xFFFF_FFFF };
        assert_eq!(c.read32(0x1010), want, "rfe {rfe}");
    }
}

#[test]
fn cut_d_on_takes_the_aux_clock_bit() {
    for (cut, set) in [(0u8, false), (2, false), (3, true), (5, true)] {
        let c = Chip::new();
        after_mac(&c, cut);
        assert_eq!(c.read32(0x03FC) & 1 << 26 != 0, set, "cut {cut}");
    }
}
