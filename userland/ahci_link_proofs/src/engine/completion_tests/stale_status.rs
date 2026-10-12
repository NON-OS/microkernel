// NONOS Operating System (AGPL-3.0-or-later)
//! ERR in PxTFD, and whose command it belongs to. On an error the HBA sets
//! PxIS.TFES and leaves the slot's PxCI bit set (AHCI 1.3.1, 6.2.2); PxTFD
//! changes only when a FIS arrives (3.3.8). So while slot 0 is outstanding
//! with no error in PxIS, an ERR there is the previous command's status, and
//! only once the HBA clears the slot bit has it posted this command's (5.6.2).

use super::model_port::{done, Snap, CLEAN_DONE, RUNNING};
use crate::error::AhciError;

/// Slot 0 issued after a command that failed: PxTFD still holds that one's
/// status, ERR set, until the drive posts a FIS for this one.
const RUNNING_AFTER_ERR: Snap = Snap { ci: 1, sact: 0, is: 0, tfd: 0x51 };

#[test]
fn a_task_file_error_fails_the_command() {
    let failed = Snap { ci: 1, sact: 0, is: 1 << 30, tfd: 0x0451 };
    for done_after in 0..24 {
        assert_eq!(done(RUNNING, failed, done_after), Err(AhciError::CommandFailed));
        assert_eq!(done(RUNNING_AFTER_ERR, failed, done_after), Err(AhciError::CommandFailed));
    }
    // An HBA that clears the slot with the error, as older QEMU releases do.
    let cleared = Snap { ci: 0, sact: 0, is: 0, tfd: 0x51 };
    assert_eq!(done(RUNNING, cleared, 5), Err(AhciError::CommandFailed));
}

#[test]
fn err_left_by_the_previous_command_is_not_this_ones() {
    /*
     * With no PxIS error the slot is still running: the wait goes on, and
     * where nothing else ever changes it times out, never fails the command
     * on another command's status.
     */
    assert_eq!(done(RUNNING_AFTER_ERR, RUNNING_AFTER_ERR, usize::MAX), Err(AhciError::Timeout));
    for done_after in 0..24 {
        let got = done(RUNNING_AFTER_ERR, CLEAN_DONE, done_after);
        assert_eq!(got, Ok(()), "stale ERR for {done_after} reads, then a clean end");
    }
}
