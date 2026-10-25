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

//! The receiver on, last. On the 82599 the security block's receive path is
//! held while RXCTRL.RXEN is written (an 82599 erratum, as observed,
//! unverified); the X540 and X550 do not need it. If the
//! security block does not report ready within its bound the receiver goes
//! on anyway: holding the port down would not make it ready.

use crate::bring::wait::until;
use crate::facts::bounds::SECRX_MS;
use crate::facts::mac::{RXCTRL_RXEN, SECRXCTRL_RX_DIS, SECRXSTAT_RDY};
use crate::facts::parts::Part;
use crate::facts::regs::{REG_RXCTRL, REG_SECRXCTRL, REG_SECRXSTAT};
use crate::regs::Regs;

pub fn enable(regs: &Regs, part: Part) {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (facts::regs).
    let held = part.is_82599().then(|| unsafe {
        let sec = regs.r32(REG_SECRXCTRL);
        regs.w32(REG_SECRXCTRL, sec | SECRXCTRL_RX_DIS);
        sec
    });
    if held.is_some() {
        // SAFETY: as above.
        let _ = until(SECRX_MS, || unsafe { regs.r32(REG_SECRXSTAT) } & SECRXSTAT_RDY != 0);
    }
    // SAFETY: as above.
    unsafe {
        let rxctrl = regs.r32(REG_RXCTRL);
        regs.w32(REG_RXCTRL, rxctrl | RXCTRL_RXEN);
        if let Some(sec) = held {
            regs.w32(REG_SECRXCTRL, sec & !SECRXCTRL_RX_DIS);
        }
    }
}
