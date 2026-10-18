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

//! Stop, reset, and the software style. A part that firmware left running
//! (a PXE boot ROM) is stopped first so its DMA ends before the reset; out
//! of the reset it must read CSR0 as STOP alone, which is also how Linux
//! tells a PCnet in word I/O from anything else at the same address.

use nonos_libc::Deadline;

use crate::constants::csr::{BCR20, CSR0, CSR0_STOP, SWSTYLE_PCNET_PCI};
use crate::regs::Regs;

use super::hold::hold_ms;

/// The data sheets give the reset a microsecond; a millisecond covers it.
const RESET_SETTLE_MS: u64 = 1;
/// Bound on the part answering STOP after the reset.
const RESET_WAIT_MS: u64 = 20;

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    regs.set_csr(CSR0, CSR0_STOP);
    regs.reset();
    hold_ms(RESET_SETTLE_MS);
    let deadline = Deadline::after_ms(RESET_WAIT_MS);
    while regs.csr(CSR0) != CSR0_STOP {
        if deadline.expired() {
            return Err("part did not come out of reset stopped");
        }
        core::hint::spin_loop();
    }
    // Written while stopped, which is the only time the part takes it: 32-bit
    // addresses and the PCnet-PCI descriptor layout the rings use.
    regs.set_bcr(BCR20, SWSTYLE_PCNET_PCI);
    Ok(())
}
