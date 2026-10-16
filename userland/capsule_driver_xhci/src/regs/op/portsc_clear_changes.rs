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
use super::portsc_addr::portsc_addr;
use super::portsc_neutral::portsc_neutral;
use crate::constants::PORTSC_CHANGE_BITS;
use crate::regs::mmio_write32;

// Acknowledge the change bits set in `snapshot`. The rest of the write is
// the neutral value: power, link state and wake enables written back as
// read, and never a one into PR (reset), WPR (warm reset) or PED (writing 1
// disables the port).
pub fn portsc_clear_changes(op_base: u64, port: u8, snapshot: u32) {
    let to_clear = snapshot & PORTSC_CHANGE_BITS;
    let Some(reg) = portsc_addr(op_base, port).filter(|_| to_clear != 0) else { return };
    // SAFETY: `read_layout` checked every root port's register set lies inside the mapped window
    // (`require_ports`), and the callers pass a port from 1 to the controller's count.
    unsafe { mmio_write32(reg, portsc_neutral(snapshot) | to_clear) };
}
