// NONOS Operating System (AGPL-3.0-or-later)
//! A device error on one command, then the next command. The error leaves
//! ERR in PxTFD, and PxTFD changes only when the drive posts a FIS, so the
//! next command's first looks still read it while the drive works on it.

use crate::constants::ata::SECTOR_SIZE;
use crate::engine::transfer::transfer;
use crate::error::AhciError;
use crate::model::{pattern, Outcome, Rig, Step};

/// What `n` sectors from `lba` read back as.
fn sectors(lba: u64, n: u64) -> Vec<u8> {
    (lba..lba + n).flat_map(pattern).collect()
}

#[test]
fn a_device_error_fails_its_own_command_only() {
    for ms in [1, 5, 40] {
        let unc = Step { ms: 1, outcome: Outcome::Error(0x40) };
        let next = Step { ms, outcome: Outcome::Clean };
        let mut rig = Rig::new(vec![unc, next], true);
        let regs = rig.regs();
        let r = transfer(&mut rig.port, regs, 100, 8, false);
        assert_eq!(r, Err(AhciError::CommandFailed));
        assert_eq!(rig.port.last_tfd & 0xffff, 0x4051, "the drive's own status and error");
        let r = transfer(&mut rig.port, regs, 200, 8, false);
        assert_eq!(r, Ok(()), "the read after a failed one, taking {ms} ms");
        assert_eq!(rig.data(8 * SECTOR_SIZE), sectors(200, 8));
        assert_eq!(rig.hba().drive.taken.len(), 2);
    }
}
