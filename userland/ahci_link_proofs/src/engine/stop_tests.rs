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

//! The engine stop that parks a port, on its registers in host memory.

use super::recover_tests::{at, PORT_WORDS};
use crate::constants::regs::{CMD_FRE, CMD_ST, PORT_CMD};
use crate::regs::Regs;

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
