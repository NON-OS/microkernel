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

//! The MAC-wide receive and transmit settings, before the rings: CRC strip,
//! transmit CRC and padding in HLREG0; the RDRXCTL bits the 82599 and later
//! want (facts::mac), with its CRC strip matching HLREG0's; RSC's ACK
//! coalescing off, since RSC is never used; and broadcast accepted.
//!
//! JUMBOEN is cleared: the buffers are 2 KiB and the protocol caps a frame at
//! 1514 bytes, so a long frame is refused by the MAC instead of spilling into
//! a second descriptor.

use crate::facts::mac::*;
use crate::facts::regs::{REG_FCTRL, REG_HLREG0, REG_RDRXCTL, REG_RSCDBU};
use crate::regs::Regs;

pub fn program(regs: &Regs) {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (facts::regs).
    unsafe {
        let hl = regs.r32(REG_HLREG0) & !HLREG0_JUMBOEN;
        regs.w32(REG_HLREG0, hl | HLREG0_TXCRCEN | HLREG0_RXCRCSTRP | HLREG0_TXPADEN);
        let dbu = regs.r32(REG_RSCDBU);
        regs.w32(REG_RSCDBU, dbu | RSCDBU_RSCACKDIS);
        let mut rdrx = regs.r32(REG_RDRXCTL) & !RDRXCTL_RSCFRSTSIZE;
        rdrx |= RDRXCTL_RSCACKC | RDRXCTL_FCOE_WRFIX | RDRXCTL_CRCSTRIP;
        regs.w32(REG_RDRXCTL, rdrx);
        let fctrl = regs.r32(REG_FCTRL);
        regs.w32(REG_FCTRL, fctrl | FCTRL_BAM);
    }
}
