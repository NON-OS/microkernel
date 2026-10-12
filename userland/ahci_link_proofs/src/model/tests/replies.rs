// NONOS Operating System (AGPL-3.0-or-later)
//! What a failed command answers its client. Only a command the drive
//! itself failed (TFES, ERR set in PxTFD) answers in the ATA status range,
//! since only then is the status the disk's. One the HBA failed with an
//! error of its own, or the link changed under, answers an I/O error:
//! PxTFD then holds whatever the disk last posted, not this command's end.

use super::failure::status;
use crate::engine::flush::flush;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::{Outcome, Rig, Step};
use crate::protocol::{ata_status, E_IO};

#[test]
fn an_hba_error_answers_an_io_error() {
    // IFS, HBDS, HBFS, OFS, and IFS with TFES.
    for bits in [1u32 << 27, 1 << 28, 1 << 29, 1 << 24, 1 << 27 | 1 << 30] {
        let mut rig = Rig::new(vec![Step { ms: 40, outcome: Outcome::Clean }], true);
        let regs = rig.regs();
        rig.hba().quirks.fatal_on_send = Some(bits);
        let r = transfer(&mut rig.port, regs, 1_000, 8, false);
        assert_eq!(r.map_err(|e| status(&rig.port, e)), Err(E_IO), "PxIS {bits:#x}");
    }
}

#[test]
fn a_link_change_answers_an_io_error() {
    for write in [false, true] {
        let mut rig = Rig::new(vec![Step { ms: 5_000, outcome: Outcome::Clean }], true);
        let regs = rig.regs();
        rig.hba().quirks.self_reset_ms = Some(100);
        let r = if write {
            rig.load(&[0x77; 512]);
            transfer(&mut rig.port, regs, 900, 1, true)
        } else {
            flush(&mut rig.port, regs)
        };
        assert_eq!(r.map_err(|e| status(&rig.port, e)), Err(E_IO), "write {write}");
    }
}

#[test]
fn a_disk_error_answers_the_disks_status() {
    for err in [0x04u8, 0x10, 0x40, 0x84] {
        let mut rig = Rig::new(vec![Step { ms: 1, outcome: Outcome::Error(err) }], true);
        let regs = rig.regs();
        let r = transfer(&mut rig.port, regs, 100, 8, false);
        let want = ata_status(u32::from(err) << 8 | 0x51);
        assert_eq!(r.map_err(|e| status(&rig.port, e)), Err(want), "error {err:#x}");
    }
    // A status without ERR is no disk error, whatever said the command failed.
    let mut rig = Rig::new(Vec::new(), true);
    for tfd in [0x50, 0x80, 0xd0, 0x58] {
        rig.port.last_tfd = tfd;
        assert_eq!(status(&rig.port, AhciError::CommandFailed), E_IO, "PxTFD {tfd:#x}");
    }
}
