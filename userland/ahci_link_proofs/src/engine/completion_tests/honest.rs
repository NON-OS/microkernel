// NONOS Operating System (AGPL-3.0-or-later)
//! A port that answers as the spec says it will.

use super::model_port::{done, rounds_of, ModelPort, Snap, CLEAN_DONE, LIMIT, RUNNING};
use crate::engine::ready::wait_ready;
use crate::error::AhciError;

#[test]
fn a_clean_completion_is_done_wherever_it_lands() {
    for done_after in 0..24 {
        assert_eq!(done(RUNNING, CLEAN_DONE, done_after), Ok(()), "done after {done_after} reads");
    }
}

#[test]
fn a_command_the_device_never_finishes_times_out() {
    assert_eq!(done(RUNNING, RUNNING, usize::MAX), Err(AhciError::Timeout));
}

#[test]
fn each_fatal_interrupt_status_fails_the_command() {
    // IFS, HBDS, HBFS, TFES, and the link changes PRCS and PCS.
    for bit in [27u32, 28, 29, 30, 22, 6] {
        let fatal = Snap { ci: 1, sact: 0, is: 1 << bit, tfd: 0x80 };
        assert_eq!(done(RUNNING, fatal, 3), Err(AhciError::CommandFailed), "PxIS bit {bit}");
    }
}

#[test]
fn a_device_stuck_busy_is_never_issued_to() {
    for tfd in [0x80u32, 0x08, 0x88, 0xff] {
        let busy = Snap { ci: 0, sact: 0, is: 0, tfd };
        let mut port = ModelPort::new(busy, busy, usize::MAX);
        assert_eq!(
            wait_ready(|off| port.read(off), rounds_of(LIMIT)),
            Err(AhciError::Timeout),
            "{tfd:#x}"
        );
    }
}

#[test]
fn a_device_that_settles_is_issued_to() {
    let busy = Snap { ci: 0, sact: 0, is: 0, tfd: 0x80 };
    let idle = Snap { ci: 0, sact: 0, is: 0, tfd: 0x50 };
    for done_after in 0..8 {
        let mut port = ModelPort::new(busy, idle, done_after);
        assert_eq!(
            wait_ready(|off| port.read(off), rounds_of(LIMIT)),
            Ok(()),
            "idle after {done_after}"
        );
    }
}

#[test]
fn a_port_whose_engine_is_off_is_never_issued_to() {
    // With ST clear the HBA never fetches PxCI (AHCI 1.3.1, 3.3.7); with CR
    // still clear the engine has not started yet.
    let idle = Snap { ci: 0, sact: 0, is: 0, tfd: 0x50 };
    for cmd in [0u32, 1 << 0, 1 << 15, 1 << 4 | 1 << 14] {
        let mut port = ModelPort::new(idle, idle, usize::MAX);
        port.cmd = cmd;
        let got = wait_ready(|off| port.read(off), rounds_of(LIMIT));
        assert_eq!(got, Err(AhciError::Timeout), "PxCMD {cmd:#x}");
    }
}
