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
use crate::regs::mmio_write32;
pub fn portsc_write(op_base: u64, port: u8, value: u32) {
    let Some(reg) = portsc_addr(op_base, port) else { return };
    // SAFETY: `read_layout` checked every root port's register set lies inside the mapped window
    // (`require_ports`), and the callers pass a port from 1 to the controller's count.
    unsafe { mmio_write32(reg, value) };
}
