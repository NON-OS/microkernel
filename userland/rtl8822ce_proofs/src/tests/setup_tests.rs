// NONOS Operating System (AGPL-3.0-or-later)
//! The PHY bring-up: the front ends rtw88 defines (0 to 6) and no other;
//! the table condition with rtw88's defaults; the crystal in both halves;
//! the CCK gain bounds; and the whole of it in rtw8822c_phy_set_param's
//! order on the modelled chip, its path count from the efuse info.

use super::fixtures::{board, last, wrote};
use crate::model::Chip;
use crate::phy::bring_up;
use crate::phy::rf::paths;
use crate::phy::setup::{cck_gain_bounds, cond, rfe_is_supported, xtal_word};

#[test]
fn only_rtw88s_front_ends_are_taken() {
    assert!((0..=6).all(rfe_is_supported));
    assert!(!rfe_is_supported(7) && !rfe_is_supported(0x15) && !rfe_is_supported(0xFF));
}

#[test]
fn the_condition_takes_rtw88s_defaults() {
    let c = cond(&board(0, 0));
    assert_eq!((c.cut, c.pkg, c.intf, c.rfe), (15, 15, 1, 0));
    let c = cond(&board(4, 6));
    assert_eq!((c.cut, c.rfe), (4, 6));
}

#[test]
fn the_crystal_fills_both_halves() {
    assert_eq!(xtal_word(0x25), 0x25 | 0x25 << 7);
    assert_eq!(xtal_word(0xA5), 0x25 | 0x25 << 7, "seven bits");
}

#[test]
fn the_cck_bounds_are_read_from_the_agc() {
    let c = Chip::new();
    c.set32(0x1A98, 0x0000_8080); // upper MSB 2 (bits 15:14), lower MSB 2 (7:6)
    c.set32(0x1AA8, 0x0005_0000); // upper LSB 5
    c.set32(0x1A70, 0x0300_0000); // lower LSB 3
    assert_eq!(cck_gain_bounds(&c), (0x25, 0x23));
}

#[test]
fn the_phy_comes_up_in_rtw88s_order() {
    let _one = super::lock();
    let c = Chip::new();
    bring_up(&c, &board(3, 0));
    assert_eq!(paths(), 2);
    let first = |a| c.first(a).unwrap_or(usize::MAX);
    assert!(first(0x0002) < first(0x180C), "BB/RF power, then the header init");
    assert!(first(0x180C) < first(0x1D0C), "the header init, then the tables");
    assert!(last(&c, 0x1B80) < Some(first(0x1040)), "the calibration table, then the crystal");
    assert!(first(0x4108) < first(0x1040), "the RF tables, then the crystal");
    assert_eq!(c.get32(0x1040) & 0x00FF_FC00, xtal_word(0x25) << 10);
    let antmap = c.writes.borrow().iter().rposition(|w| w.0 == 0x0820 && w.2 & 0xFF == 0x31);
    let paths_set = antmap.expect("1SS on A of AB");
    assert!(first(0x1040) < paths_set, "the crystal, then the paths");
    assert!(Some(paths_set) < wrote(&c, 0x1C3C, 0xA_8003), "then the DAC calibration");
    assert_eq!(c.get32(0x0A70) & 0x3FF, 0x3FF, "DPD off at the end");
    bring_up(&Chip::new(), &crate::efuse::EfuseInfo { paths: 1, ..board(3, 0) });
    assert_eq!(paths(), 1, "a one-path chip");
    crate::phy::rf::set_paths(2);
}
