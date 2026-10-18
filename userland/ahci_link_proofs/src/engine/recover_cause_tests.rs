// NONOS Operating System (AGPL-3.0-or-later)
//! Recovery by cause, on the model port. CLO clears only the HBA's copy of
//! BSY and DRQ, so only a COMRESET stops the drive, before a command was
//! issued or after, and CLO is never set. Every COMRESET holds DET at 1 for
//! at least 1 ms (AHCI 1.3.1, 3.3.11), or the PHY never sends it: the model
//! counts a shorter hold and the tests refuse it.

use std::time::Duration;

use super::cause::Cause;
use super::recover::recover;
use crate::clock::now;
use crate::error::AhciError;
use crate::model::Rig;

/// Recover a port whose PxTFD reads `tfd`; gives back the result, the
/// COMRESETs the drive saw, the CLO requests, and how long it took.
pub(super) fn recovered(
    tfd: u32,
    sclo: bool,
    cause: Cause,
    link_ms: u64,
) -> (Result<(), AhciError>, u32, u32, Duration) {
    let mut rig = Rig::new(Vec::new(), sclo);
    (rig.hba().reg.tfd, rig.hba().quirks.link_ms) = (tfd, link_ms);
    let (t, regs) = (now(), rig.regs());
    let r = recover(&mut rig.port, regs, cause);
    let took = now() - t;
    let hba = rig.hba();
    assert_eq!(hba.count.short_holds, 0, "a COMRESET held under 1 ms");
    (r, hba.count.comresets, hba.count.clo, took)
}

#[test]
fn an_abandoned_command_is_ended_by_comreset_whatever_ptfd_reads() {
    for tfd in [0x50, 0x51, 0x80, 0xd0, 0x58] {
        for sclo in [true, false] {
            let (r, resets, clo, _) = recovered(tfd, sclo, Cause::Abandoned, 10);
            assert_eq!((r, resets, clo), (Ok(()), 1, 0), "PxTFD {tfd:#x}, SCLO {sclo}");
        }
    }
}

#[test]
fn an_error_the_drive_ended_needs_no_reset() {
    for sclo in [true, false] {
        let (r, resets, clo, _) = recovered(0x4051, sclo, Cause::Failed, 10);
        assert_eq!((r, resets, clo), (Ok(()), 0, 0), "SCLO {sclo}");
    }
}

#[test]
fn a_failed_command_still_busy_is_reset_never_overridden() {
    for tfd in [0x80, 0xd0, 0x58, 0x88] {
        let (r, resets, clo, _) = recovered(tfd, true, Cause::Failed, 10);
        assert_eq!((r, resets, clo), (Ok(()), 1, 0), "PxTFD {tfd:#x}");
    }
}

#[test]
fn a_port_stuck_before_issue_is_reset_never_overridden() {
    for tfd in [0x80, 0xd0, 0x58, 0x88] {
        for sclo in [true, false] {
            let (r, resets, clo, _) = recovered(tfd, sclo, Cause::Stuck, 10);
            assert_eq!((r, resets, clo), (Ok(()), 1, 0), "PxTFD {tfd:#x}, SCLO {sclo}");
        }
    }
}
