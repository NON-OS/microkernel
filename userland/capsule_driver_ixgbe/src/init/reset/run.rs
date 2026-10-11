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

//! The reset in ixgbe_reset_hw's order: the adapter stopped, the PCIe master
//! stopped, then the MAC reset. AUTOC is read first, so the 82599's link
//! mode can be put back after it (`init::autoc`).

use crate::constants::regs::REG_AUTOC;
use crate::regs::Regs;

use super::{global, master, quiesce};

/// AUTOC as it was before the reset.
pub fn run(regs: &Regs) -> Result<u32, &'static str> {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and AUTOC
    // is a 32-bit register inside it (ixgbe_type.h).
    let autoc = unsafe { regs.r32(REG_AUTOC) };
    quiesce::run(regs);
    master::stop(regs)?;
    global::run(regs)?;
    Ok(autoc)
}
