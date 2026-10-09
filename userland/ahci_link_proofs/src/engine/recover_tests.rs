// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The recovery a failed command runs, on one port's registers held in host
//! memory. Plain memory never sets CR or FR, so the engine waits in stop and
//! start end at once and only the bits the driver writes are left to read.

use super::recover::recover;
use crate::constants::regs::{
    CMD_CLO, CMD_FRE, CMD_ST, PORT_CMD, PORT_IS, PORT_SCTL, PORT_SERR, PORT_TFD, SCTL_DET_MASK,
};
use crate::error::AhciError;
use crate::regs::Regs;

const PORT_WORDS: usize = 0x80 / 4;

fn at(off: u32) -> usize {
    off as usize / 4
}

/// Run recover on a port whose PxCMD reads `cmd`, and give back PxCMD.
fn recover_from(cmd: u32) -> u32 {
    let mut file = [0u32; PORT_WORDS];
    file[at(PORT_CMD)] = cmd;
    file[at(PORT_SERR)] = 1 << 26;
    file[at(PORT_IS)] = 1 << 30;
    let base = file.as_mut_ptr();
    assert_eq!(recover(Regs::new(base as u64), 0, true), Ok(()));
    // SAFETY: `base` points at `file`, which outlives the read.
    unsafe { core::ptr::read_volatile(base.add(at(PORT_CMD))) }
}

#[test]
fn recovery_restarts_the_engine_with_fis_receive_on() {
    /*
     * ST may be set only with FRE set (AHCI 1.3.1, 3.3.7). Without FIS
     * receive the HBA posts no D2H FIS, so PxTFD would keep the failed
     * command's ERR and every later command on the port would fail.
     */
    for cmd in [CMD_ST | CMD_FRE, CMD_ST, CMD_FRE, 0] {
        let after = recover_from(cmd);
        assert_eq!(after & CMD_FRE, CMD_FRE, "PxCMD {cmd:#x}: FIS receive left off");
        assert_eq!(after & CMD_ST, CMD_ST, "PxCMD {cmd:#x}: engine not restarted");
    }
}

/// Run recover on a port whose device holds `tfd` and whose PHY reads no
/// link (PxSSTS 0), with or without CAP.SCLO. Gives back the result, PxCMD
/// and PxSCTL.
fn recover_stuck(tfd: u32, sclo: bool) -> (Result<(), AhciError>, u32, u32) {
    let mut file = [0u32; PORT_WORDS];
    file[at(PORT_CMD)] = CMD_ST | CMD_FRE;
    file[at(PORT_TFD)] = tfd;
    let base = file.as_mut_ptr();
    let r = recover(Regs::new(base as u64), 0, sclo);
    // SAFETY: `base` points at `file`, which outlives the reads.
    unsafe {
        (
            r,
            core::ptr::read_volatile(base.add(at(PORT_CMD))),
            core::ptr::read_volatile(base.add(at(PORT_SCTL))),
        )
    }
}

#[test]
fn a_port_stuck_busy_is_kicked_by_clo_then_comreset() {
    /*
     * Plain memory never clears CLO nor BSY, so CLO is seen to be tried and
     * to fail, and the COMRESET after it finds no link: the port reads empty,
     * and the engine is restarted all the same.
     */
    for tfd in [0x80u32, 0x08, 0x88] {
        let (r, cmd, sctl) = recover_stuck(tfd, true);
        assert_eq!(r, Err(AhciError::NoDisk), "TFD {tfd:#x}");
        assert_eq!(cmd & CMD_CLO, CMD_CLO, "TFD {tfd:#x}: CLO never tried");
        assert_eq!(cmd & (CMD_ST | CMD_FRE), CMD_ST | CMD_FRE, "TFD {tfd:#x}: engine left off");
        assert_eq!(sctl & SCTL_DET_MASK, 0, "TFD {tfd:#x}: PHY left in COMRESET");
    }
}

#[test]
fn without_sclo_a_stuck_port_gets_comreset_alone() {
    let (r, cmd, sctl) = recover_stuck(0x80, false);
    assert_eq!(r, Err(AhciError::NoDisk));
    assert_eq!(cmd & CMD_CLO, 0, "CLO set on an HBA without CAP.SCLO");
    assert_eq!(sctl & SCTL_DET_MASK, 0);
}

#[test]
fn an_idle_port_is_not_kicked() {
    let (r, cmd, sctl) = recover_stuck(0x50, true);
    assert_eq!(r, Ok(()));
    assert_eq!(cmd & CMD_CLO, 0);
    assert_eq!(sctl, 0, "COMRESET sent to a port that was not stuck");
}

#[test]
fn a_stopped_port_has_engine_and_fis_receive_off() {
    for cmd in [CMD_ST | CMD_FRE, CMD_ST, CMD_FRE, 0, !(1u32 << 14 | 1 << 15)] {
        let mut file = [0u32; PORT_WORDS];
        file[at(PORT_CMD)] = cmd;
        let base = file.as_mut_ptr();
        assert_eq!(super::stop::stop(Regs::new(base as u64), 0), Ok(()));
        // SAFETY: `base` points at `file`, which outlives the read.
        let after = unsafe { core::ptr::read_volatile(base.add(at(PORT_CMD))) };
        assert_eq!(after & (CMD_ST | CMD_FRE), 0, "PxCMD {cmd:#x}");
    }
}
