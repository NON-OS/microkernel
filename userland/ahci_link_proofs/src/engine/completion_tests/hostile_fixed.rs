// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile endings chosen by hand: each wherever it lands between reads.

use super::hostile::OFS;
use super::model_port::{done, Snap, CLEAN_DONE, RUNNING, RUNNING_STALE};
use crate::error::AhciError;

#[test]
fn an_error_posted_with_the_completion_is_never_done() {
    /*
     * The device fails the command and the HBA clears the slot bit with
     * it, as older QEMU releases do. Wherever the completion lands between
     * two reads, ERR is seen.
     */
    let failed = [
        Snap { ci: 0, sact: 0, is: 1 << 30, tfd: 0x0451 },
        Snap { ci: 0, sact: 0, is: 0, tfd: 0x51 },
    ];
    for before in [RUNNING, RUNNING_STALE] {
        for after in failed {
            for done_after in 0..24 {
                let got = done(before, after, done_after);
                assert_eq!(
                    got,
                    Err(AhciError::CommandFailed),
                    "{before:?} then {after:?} at {done_after}"
                );
            }
        }
    }
}

#[test]
fn a_slot_cleared_while_the_device_shows_bsy_or_drq_is_not_done() {
    for tfd in [0x80u32, 0xd0, 0x08, 0x58, 0x88] {
        let unsettled = Snap { ci: 0, sact: 0, is: 1, tfd };
        for done_after in 0..12 {
            assert_eq!(done(RUNNING, unsettled, done_after), Err(AhciError::Timeout), "{tfd:#x}");
        }
        assert_eq!(done(unsettled, CLEAN_DONE, 9), Ok(()), "{tfd:#x} settling to idle is done");
    }
}

#[test]
fn a_slot_the_driver_did_not_issue_fails_the_command() {
    for bit in 1..32 {
        let stray = Snap { ci: 1 << bit, sact: 0, is: 1, tfd: 0x50 };
        assert_eq!(done(RUNNING, stray, 3), Err(AhciError::CommandFailed), "PxCI bit {bit}");
        let both = Snap { ci: 1 | 1 << bit, sact: 0, is: 0, tfd: 0x80 };
        assert_eq!(done(RUNNING, both, 3), Err(AhciError::CommandFailed), "slot 0 and {bit}");
    }
}

#[test]
fn a_queued_command_the_driver_never_issued_fails() {
    for bit in 0..32 {
        let queued = Snap { ci: 0, sact: 1 << bit, is: 1, tfd: 0x50 };
        assert_eq!(done(RUNNING, queued, 3), Err(AhciError::CommandFailed), "PxSACT bit {bit}");
    }
}

#[test]
fn an_overflow_fails_the_command() {
    let overflow = Snap { ci: 0, sact: 0, is: OFS | 1, tfd: 0x50 };
    for done_after in 0..12 {
        assert_eq!(done(RUNNING, overflow, done_after), Err(AhciError::CommandFailed));
    }
}
