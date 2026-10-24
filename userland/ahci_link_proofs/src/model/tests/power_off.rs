// NONOS Operating System (AGPL-3.0-or-later)
//! The power-off: FLUSH CACHE EXT, then STANDBY IMMEDIATE whatever the flush
//! did, each bounded by the command timeout; the kernel's alone.

use crate::constants::ata::{ATA_FLUSH_EXT, ATA_STANDBY_IMMEDIATE};
use crate::engine::flush::{flush, standby};
use crate::error::AhciError;
use crate::medium_rule::allows;
use crate::model::{Outcome, Rig, Step};
use crate::power_off_rule::power_off;
use crate::protocol::{OP_FLUSH, OP_POWER_OFF};

fn ops(rig: &Rig) -> Vec<u8> {
    rig.hba().drive.taken.iter().map(|c| c.op).collect()
}

#[test]
fn the_disk_is_flushed_then_put_in_standby() {
    let mut rig = Rig::new(Vec::new(), true);
    let regs = rig.regs();
    let done = power_off(&mut rig.port, |p| flush(p, regs), |p| standby(p, regs));
    assert_eq!(done.failure(), Ok(()));
    assert_eq!(ops(&rig).last_chunk::<2>(), Some(&[ATA_FLUSH_EXT, ATA_STANDBY_IMMEDIATE]));
}

#[test]
fn a_flush_that_never_ends_still_lets_the_standby_go() {
    let late = Step { ms: 40_000, outcome: Outcome::Clean };
    let mut rig = Rig::new(vec![late], true);
    let regs = rig.regs();
    let done = power_off(&mut rig.port, |p| flush(p, regs), |p| standby(p, regs));
    assert_eq!(done.flushed, Err(AhciError::Timeout));
    assert_eq!(done.standby, Ok(()), "the reset before it let the standby through");
    assert_eq!(done.failure(), Err(AhciError::Timeout), "the flush's failure is the reply");
    assert_eq!(ops(&rig).last(), Some(&ATA_STANDBY_IMMEDIATE));
}

#[test]
fn a_standby_the_disk_refuses_is_the_reply() {
    let refuse = Step { ms: 1, outcome: Outcome::Error(0x04) };
    let mut rig = Rig::new(vec![Step { ms: 1, outcome: Outcome::Clean }, refuse], true);
    let regs = rig.regs();
    let done = power_off(&mut rig.port, |p| flush(p, regs), |p| standby(p, regs));
    assert_eq!(done.flushed, Ok(()));
    assert!(done.failure().is_err());
}

#[test]
fn the_standby_carries_no_lba_and_no_data() {
    let mut rig = Rig::new(Vec::new(), true);
    let regs = rig.regs();
    assert_eq!(standby(&mut rig.port, regs), Ok(()));
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
