// NONOS Operating System (AGPL-3.0-or-later)
//! The tables are rtw8822c_table.c's word streams (their lengths and their
//! ends here; `tools/nonos-rtw88-tables --check` holds every word), loaded in
//! rtw88's order: baseband, AGC, the calibration init behind its clock
//! enables, then the path-B table and the path-A one, one per RF path.

use crate::model::Chip;
use crate::phy::apply::load_all;
use crate::phy::cond::{PhyCond, INTF_PCIE};
use crate::phy::rf::set_paths;
use crate::phy::tables::{RTL8822C_AGC, RTL8822C_BB, RTL8822C_CAL_INIT};
use crate::phy::tables::{RTL8822C_RF_A, RTL8822C_RF_B};

const COND: PhyCond = PhyCond { cut: 3, pkg: 15, intf: INTF_PCIE, rfe: 0 };

#[test]
fn the_tables_are_rtw88s_streams() {
    let ends = |t: &[u32]| (t.len(), t[0], t[1], t[t.len() - 2], t[t.len() - 1]);
    assert_eq!(ends(RTL8822C_BB), (3006, 0x1D0C, 0x0041_0000, 0x1EE8, 0));
    assert_eq!(ends(RTL8822C_AGC), (3728, 0x8300_0000, 0, 0xB000_0000, 0));
    assert_eq!(ends(RTL8822C_CAL_INIT), (4920, 0x1B00, 0x8, 0x1B80, 0x2));
    assert_eq!(ends(RTL8822C_RF_A), (40070, 0x000, 0x3_0000, 0x0EF, 0));
    assert_eq!(ends(RTL8822C_RF_B), (40706, 0x000, 0x3_0000, 0x0EF, 0));
}

#[test]
fn the_tables_load_in_rtw88s_order() {
    let _one = super::lock();
    set_paths(2);
    let c = Chip::new();
    load_all(&c, &COND, 2);
    let first = |a| c.first(a).unwrap_or(usize::MAX);
    let w = c.writes.borrow().clone();
    assert_eq!(w[0], (0x1D0C, 4, 0x0041_0000), "the baseband table first");
    // rtw_load_rfk_table: 0x1E24 bit 17, then 0x1CD0 bits 28, 29, 30 on and
    // 31 off, right before the calibration table's first write.
    let cal = first(0x1B00);
    assert_eq!(w[cal - 5].0, 0x1E24);
    assert_eq!(w[cal - 5].2 & 1 << 17, 1 << 17);
    assert!(w[cal - 4..cal].iter().all(|x| x.0 == 0x1CD0));
    assert_eq!(w[cal - 1].2 >> 28, 0b0111);
    assert!(cal < first(0x4108), "the calibration table before the RF");
    assert!(first(0x4108) < first(0x1808), "path B's table, then path A's");
    assert_eq!(c.at(0x4108)[0], 0x3_0000, "RF 0 through the serial interface");
    for marker in [0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFFE] {
        assert!(c.first(marker).is_none(), "a delay {marker:#x} is not a register");
    }
}

#[test]
fn a_one_path_chip_gets_only_rf_tbl_0() {
    let _one = super::lock();
    set_paths(1);
    let c = Chip::new();
    load_all(&c, &COND, 1);
    // rf_tbl[0] is the path-B table, refused on a chip without path B.
    assert!(c.first(0x1808).is_none() && c.first(0x4108).is_none());
    let rf = |a: &usize| (0x3C00..0x4000).contains(a) || (0x4C00..0x5000).contains(a);
    assert!(c.writes.borrow().iter().all(|w| !rf(&w.0)), "no RF window written");
    set_paths(2);
}
