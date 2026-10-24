// NONOS Operating System (AGPL-3.0-or-later)
//! The power-off: FLUSH CACHE EXT within its own budget, then STANDBY
//! IMMEDIATE with what the power-off's bound leaves, or none on a restart;
//! the kernel's alone.

use crate::constants::ata::{ATA_FLUSH_EXT, ATA_STANDBY_IMMEDIATE};
use crate::constants::power_off::{
    standby_budget, POWER_OFF_FLUSH_MS, POWER_OFF_MS, STANDBY_MIN_MS,
};
use crate::engine::flush::{flush_within, standby_within};
use crate::medium_rule::allows;
use crate::model::Rig;
use crate::power_off_rule::{power_off, PoweredOff, Standby};
use crate::protocol::{OP_FLUSH, OP_POWER_OFF};

fn ops(rig: &Rig) -> Vec<u8> {
    rig.hba().drive.taken.iter().map(|c| c.op).collect()
}

/// The power-off as the handler runs it.
pub(super) fn run(rig: &mut Rig, standby_too: bool) -> PoweredOff {
    let regs = rig.regs();
    let flush = |p: &mut _, ms| flush_within(p, regs, ms);
    power_off(&mut rig.port, standby_too, flush, |p, ms| standby_within(p, regs, ms))
}

#[test]
fn the_disk_is_flushed_then_put_in_standby() {
    let mut rig = Rig::new(Vec::new(), true);
    let done = run(&mut rig, true);
    assert_eq!((done.failure(), done.standby), (Ok(()), Standby::Done));
    assert_eq!(ops(&rig).last_chunk::<2>(), Some(&[ATA_FLUSH_EXT, ATA_STANDBY_IMMEDIATE]));
}

#[test]
fn a_restart_flushes_and_leaves_the_disk_spinning() {
    let mut rig = Rig::new(Vec::new(), true);
    let done = run(&mut rig, false);
    assert_eq!((done.failure(), done.standby), (Ok(()), Standby::NotAsked));
    assert_eq!(ops(&rig).last(), Some(&ATA_FLUSH_EXT), "a restart spun the disk down");
}

#[test]
fn each_command_has_its_power_off_budget() {
    let seen = std::cell::RefCell::new(Vec::new());
    let note = |_: &mut (), ms| {
        seen.borrow_mut().push(ms);
        Ok(())
    };
    let _ = power_off(&mut (), true, note, note);
    let flush_ok = standby_budget(true).unwrap();
    assert_eq!(*seen.borrow(), [POWER_OFF_FLUSH_MS, flush_ok], "COMMAND_MS's 30 s, or no bound");
    assert!((STANDBY_MIN_MS..POWER_OFF_MS - POWER_OFF_FLUSH_MS).contains(&flush_ok));
}

#[test]
fn the_standby_carries_no_lba_and_no_data() {
    let mut rig = Rig::new(Vec::new(), true);
    let regs = rig.regs();
    assert_eq!(standby_within(&mut rig.port, regs, 5_000), Ok(()));
    let cmd = *rig.hba().drive.taken.last().unwrap();
    assert_eq!((cmd.op, cmd.lba, cmd.count), (0xE0, 0, 0), "STANDBY IMMEDIATE, ACS-3 7.47");
}

#[test]
fn only_the_kernel_may_spin_the_disk_down() {
    assert!(allows(OP_POWER_OFF, 0, false));
    assert!(!allows(OP_POWER_OFF, 7, false));
    assert!(!allows(OP_POWER_OFF, 7, true), "a store writer may not");
    assert!(allows(OP_FLUSH, 7, true), "while it may still flush");
}
