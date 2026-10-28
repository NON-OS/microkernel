// NONOS Operating System (AGPL-3.0-or-later)

//! The firmware's start: at the override address when a region names one
//! (`FW_START_OVERRIDE`), at nothing otherwise, and only counted as running
//! once the chip sets its ready bits (`MT_TOP_MISC2_FW_N9_RDY`) within the
//! wait; the download ring given back afterwards.

use super::download_tests::run;
use super::mcu_tests::card;
use crate::fw::Step;
use crate::model::images::{patch_7922, pattern, ram, ram_7922};

#[test]
fn no_start_address_means_no_override() {
    let (mut k, c) = card();
    let r = ram(&[(0x2015c00, pattern(3, 64), 0)]);
    run(&mut k, &patch_7922(), &r).unwrap();
    assert_eq!(c.borrow().mcu.started, Some((0, 0)));
}

#[test]
fn a_start_never_said_ready_is_a_failure() {
    let (mut k, c) = card();
    c.borrow_mut().faults.never_ready = true;
    assert_eq!(run(&mut k, &patch_7922(), &ram_7922()), Err(Step::NotReady));
    assert!(c.borrow().mcu.running, "started, but never said ready");
}

#[test]
fn the_mcu_is_owned_and_l0s_off_before_the_first_byte() {
    let (mut k, c) = card();
    run(&mut k, &patch_7922(), &ram_7922()).unwrap();
    let ch = c.borrow();
    assert!(!ch.fw_own, "mt7921e_driver_own");
    assert_eq!(ch.reg(0x10194) & 1 << 8, 1 << 8, "MT_PCIE_MAC_PM_L0S_DIS");
    assert!(ch.writes.contains(&(0x9f23c, 0)), "MT_SWDEF_MODE normal");
}
