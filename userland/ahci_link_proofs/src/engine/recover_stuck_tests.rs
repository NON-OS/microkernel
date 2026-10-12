// NONOS Operating System (AGPL-3.0-or-later)
//! The recovery of a port nothing was issued to whose device holds BSY or
//! DRQ, on its registers in host memory: a COMRESET, noted in the port,
//! never Command List Override, and an idle port left alone.

use super::bare_port::bare_port;
use super::cause::Cause;
use super::recover::recover;
use super::recover_tests::{at, PORT_WORDS};
use crate::clock::Deadline;
use crate::constants::regs::{CMD_FRE, CMD_ST, PORT_CMD, PORT_SCTL, PORT_TFD, SCTL_DET_MASK};
use crate::error::AhciError;
use crate::regs::Regs;

/// PxCMD.CLO (AHCI 1.3.1, 3.3.7).
const CMD_CLO: u32 = 1 << 3;

/// Run recover on a port nothing was issued to, whose device holds `tfd`
/// and whose PHY reads no link (PxSSTS 0). Gives back the result, PxCMD,
/// PxSCTL, and the reset window the port noted.
fn recover_stuck(tfd: u32) -> (Result<(), AhciError>, u32, u32, Option<Deadline>) {
    let mut file = [0u32; PORT_WORDS];
    file[at(PORT_CMD)] = CMD_ST | CMD_FRE;
    file[at(PORT_TFD)] = tfd;
    let base = file.as_mut_ptr();
    let mut port = bare_port();
    let r = recover(&mut port, Regs::new(base as u64), Cause::Stuck);
    // SAFETY: `base` points at `file`, which outlives the reads.
    let (cmd, sctl) = unsafe {
        (
            core::ptr::read_volatile(base.add(at(PORT_CMD))),
            core::ptr::read_volatile(base.add(at(PORT_SCTL))),
        )
    };
    (r, cmd, sctl, port.reset_window)
}

#[test]
fn a_port_stuck_busy_is_reset_never_overridden() {
    /*
     * Plain memory never clears BSY, so the COMRESET finds no link: the
     * port reads empty, and the engine is restarted all the same. CLO is
     * never set: it clears only the HBA's copy of BSY and DRQ.
     */
    for tfd in [0x80u32, 0x08, 0x88, 0xff] {
        let (r, cmd, sctl, window) = recover_stuck(tfd);
        assert_eq!(r, Err(AhciError::NoDisk), "TFD {tfd:#x}");
        assert_eq!(cmd & CMD_CLO, 0, "TFD {tfd:#x}: CLO set");
        assert_eq!(cmd & (CMD_ST | CMD_FRE), CMD_ST | CMD_FRE, "TFD {tfd:#x}: engine left off");
        assert_eq!(sctl & SCTL_DET_MASK, 0, "TFD {tfd:#x}: PHY left in COMRESET");
        assert!(window.is_some_and(|w| !w.expired()), "TFD {tfd:#x}: the reset not noted");
    }
}

#[test]
fn an_idle_port_is_not_kicked() {
    for tfd in [0x00u32, 0x50, 0x51, 0x41] {
        let (r, cmd, sctl, window) = recover_stuck(tfd);
        assert_eq!(r, Ok(()), "TFD {tfd:#x}");
        assert_eq!(cmd & CMD_CLO, 0, "TFD {tfd:#x}");
        assert_eq!(sctl, 0, "TFD {tfd:#x}: COMRESET sent to a port that was not stuck");
        assert!(window.is_none(), "TFD {tfd:#x}: a reset noted that was never sent");
    }
}
