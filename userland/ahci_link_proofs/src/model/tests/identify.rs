// NONOS Operating System (AGPL-3.0-or-later)
//! IDENTIFY through the command path: the block the drive sends, read back
//! by the driver's checked copy, little-endian words and all.

use crate::engine::identify::identify;
use crate::error::AhciError;
use crate::model::{Outcome, Rig, Step, CAPACITY, MODEL};

#[test]
fn identify_reads_the_block_the_drive_sends() {
    let mut rig = Rig::new(Vec::new(), true);
    let regs = rig.regs();
    assert_eq!(identify(&mut rig.port, regs), Ok(CAPACITY));
    assert_eq!(rig.port.capacity_sectors, CAPACITY);
    assert_eq!(rig.port.names.model(), MODEL);
}

#[test]
fn identify_after_an_aborted_command_is_not_failed_by_its_status() {
    let abrt = Step { ms: 1, outcome: Outcome::Error(0x04) };
    let mut rig = Rig::new(vec![abrt], false);
    let regs = rig.regs();
    assert_eq!(identify(&mut rig.port, regs), Err(AhciError::CommandFailed));
    assert_eq!(identify(&mut rig.port, regs), Ok(CAPACITY));
}
