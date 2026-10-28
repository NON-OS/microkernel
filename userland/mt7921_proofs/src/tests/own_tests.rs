// NONOS Operating System (AGPL-3.0-or-later)

//! Ownership and the subsystem reset (`mt792xe_mcu_fw_pmctrl`,
//! `__mt792xe_mcu_drv_pmctrl`, `mt7921e_driver_own`, `mt792x_wfsys_reset`):
//! each request written, its answer awaited, ten tries at most.

use crate::model::rig::{chip, regs};
use crate::power::own::{drv_own, fw_own, mcu_drv_own};
use crate::power::{identify, wfsys_reset};

fn lpctl_writes(c: &crate::model::Shared, v: u32) -> usize {
    c.borrow().writes.iter().filter(|w| **w == (0xe0010, v)).count()
}

#[test]
fn the_firmware_then_the_driver_take_the_chip() {
    let c = chip();
    let r = regs(&c);
    assert!(fw_own(&r) && c.borrow().own_sync);
    assert!(drv_own(&r) && !c.borrow().own_sync);
    assert_eq!((lpctl_writes(&c, 1), lpctl_writes(&c, 2)), (1, 1), "SET_OWN, then CLR_OWN");
}

#[test]
fn driver_own_gives_up_after_ten_tries() {
    let c = chip();
    c.borrow_mut().faults.no_own = true;
    let r = regs(&c);
    assert!(fw_own(&r));
    assert!(!drv_own(&r));
    assert_eq!(lpctl_writes(&c, 2), 10, "MT792x_DRV_OWN_RETRY_COUNT");
}

#[test]
fn the_mcu_is_owned_through_the_l1_window() {
    let c = chip();
    assert!(mcu_drv_own(&regs(&c)));
    assert!(!c.borrow().fw_own);
    assert_eq!(c.borrow().l1.get(&0x18060010), Some(&2), "DRV_OWN in MT_TOP_LPCR_HOST_BAND0");
}

#[test]
fn the_reset_is_held_then_released_and_must_finish() {
    let c = chip();
    assert!(wfsys_reset(&regs(&c)));
    let window = |w: &&(u32, u32)| (0x40000..0x50000).contains(&w.0);
    let l1: Vec<u32> = c.borrow().writes.iter().filter(window).map(|w| w.1).collect();
    assert_eq!(l1.iter().map(|v| v & 1).collect::<Vec<_>>(), vec![0, 1], "held, released");
    let c = chip();
    c.borrow_mut().faults.no_reset = true;
    assert!(!wfsys_reset(&regs(&c)));
}

#[test]
fn identity_comes_through_the_window() {
    let c = chip();
    c.borrow_mut().id = (0x7961, 1 << 7, 0x8a);
    let (chip_id, rev) = identify(&regs(&c)).unwrap();
    assert_eq!((chip_id, rev), (crate::ids::Chip::Mt7920, 0x8a));
    c.borrow_mut().id.0 = 0x7663;
    assert!(identify(&regs(&c)).is_none());
}
