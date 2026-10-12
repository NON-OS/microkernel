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

//! The recovery a failure runs, on one port's registers held in host memory.
//! Plain memory never sets CR or FR, so the engine waits in stop and start
//! end at once and only the bits the driver writes are left to read. The
//! recovery by cause, on a model port, is in `recover_cause_tests`.

use super::bare_port::bare_port;
use super::cause::Cause;
use super::recover::recover;
use crate::constants::regs::{CMD_FRE, CMD_ST, PORT_CMD, PORT_IS, PORT_SERR};
use crate::regs::Regs;

pub(super) const PORT_WORDS: usize = 0x80 / 4;

pub(super) fn at(off: u32) -> usize {
    off as usize / 4
}

/// Run recover on a port whose PxCMD reads `cmd`, and give back PxCMD.
fn recover_from(cmd: u32) -> u32 {
    let mut file = [0u32; PORT_WORDS];
    file[at(PORT_CMD)] = cmd;
    file[at(PORT_SERR)] = 1 << 26;
    file[at(PORT_IS)] = 1 << 30;
    let base = file.as_mut_ptr();
    assert_eq!(recover(&mut bare_port(), Regs::new(base as u64), Cause::Failed), Ok(()));
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
