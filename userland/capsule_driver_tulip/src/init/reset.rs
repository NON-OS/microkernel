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

//! The software reset. A part that firmware left running (a PXE boot ROM)
//! stops with it; out of the reset both processes must read stopped, which
//! is also what tells a 21x4x apart from anything else at the address.
//! Then the bus mode, every interrupt masked and every status bit cleared:
//! the driver polls and binds no line.

use nonos_libc::Deadline;

use crate::constants::csr::{
    CSR0, CSR0_BUS_MODE, CSR0_SWR, CSR5, CSR5_CLEAR_ALL, CSR5_FBE, CSR5_RS_SHIFT, CSR5_STATE_MASK,
    CSR5_TS_SHIFT, CSR6, CSR7,
};
use crate::regs::Regs;

use super::hold::hold_ms;

/// Linux tulip waits a hundred microseconds after SWR; a millisecond covers
/// it and the part's 50 bus cycles.
const RESET_SETTLE_MS: u64 = 1;
const RESET_WAIT_MS: u64 = 20;

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    regs.set_csr(CSR6, 0);
    regs.set_csr(CSR0, CSR0_SWR);
    hold_ms(RESET_SETTLE_MS);
    let deadline = Deadline::after_ms(RESET_WAIT_MS);
    while regs.csr(CSR0) & CSR0_SWR != 0 || !stopped(regs.csr(CSR5)) {
        if deadline.expired() {
            return Err("part did not come out of reset stopped");
        }
        core::hint::spin_loop();
    }
    regs.set_csr(CSR0, CSR0_BUS_MODE);
    regs.set_csr(CSR7, 0);
    regs.set_csr(CSR5, CSR5_CLEAR_ALL);
    Ok(())
}

/// Both processes stopped in a CSR5 value.
pub const fn stopped(csr5: u32) -> bool {
    (csr5 >> CSR5_RS_SHIFT) & CSR5_STATE_MASK == 0 && (csr5 >> CSR5_TS_SHIFT) & CSR5_STATE_MASK == 0
}

/// Both processes running (any state but stopped) and no fatal bus error.
pub const fn running(csr5: u32) -> bool {
    let rx = (csr5 >> CSR5_RS_SHIFT) & CSR5_STATE_MASK != 0;
    let tx = (csr5 >> CSR5_TS_SHIFT) & CSR5_STATE_MASK != 0;
    rx && tx && csr5 & CSR5_FBE == 0
}
